const SOL_MINT = "So11111111111111111111111111111111111111112";
const USDC_MINT = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";
const JUP_MINT = "JUPyiwrYJFskUPiHa7hkeR8VUtAeFoSYbKedZNsDvCN";
const state = {
  pulse: null,
  selectedAddress: "",
  timer: null,
};

const ids = {
  status: document.getElementById("dex-status"),
  marketStrip: document.getElementById("market-strip"),
  memeFlow: document.getElementById("meme-flow"),
  freshListings: document.getElementById("fresh-listings"),
  newPairs: document.getElementById("new-pairs"),
  recentSwaps: document.getElementById("recent-swaps"),
  largeTrades: document.getElementById("large-trades"),
  quoteResult: document.getElementById("quote-result"),
  analysisAddress: document.getElementById("analysis-address"),
  analysisOutput: document.getElementById("analysis-output"),
  analysisState: document.getElementById("analysis-state"),
  birdeyeLink: document.getElementById("birdeye-link"),
  jupiterLink: document.getElementById("jupiter-link"),
};

document.getElementById("refresh-button").addEventListener("click", () => loadPulse({ manual: true }));
document.getElementById("auto-refresh").addEventListener("change", (event) => configureTimer(event.currentTarget.checked));
document.getElementById("flip-button").addEventListener("click", flipPair);
document.querySelectorAll("[data-pair]").forEach((button) => button.addEventListener("click", () => setPair(button.dataset.pair)));
document.getElementById("copy-address").addEventListener("click", copySelectedAddress);
document.getElementById("swap-form").addEventListener("submit", quoteSwap);
document.getElementById("analyze-form").addEventListener("submit", (event) => {
  event.preventDefault();
  analyzeToken(ids.analysisAddress.value);
});

await loadPulse();
configureTimer(true);

async function loadPulse({ manual = false } = {}) {
  setStatus(manual ? "Refreshing..." : "Connecting", "pending");
  try {
    const pulse = await getJson("/api/swap/pulse");
    state.pulse = pulse;
    renderPulse(pulse);
    const mode = pulse.provider?.birdeye?.status === "live" ? "Live" : "Degraded";
    const suffix = pulse.cached ? "cached" : formatAge(pulse.generatedAt);
    setStatus(`${mode} / ${suffix}`, pulse.ok ? "ok" : "warn");
  } catch (error) {
    setStatus(`Offline / ${error.message}`, "error");
    renderError(error.message);
  }
}

function configureTimer(enabled) {
  if (state.timer) clearInterval(state.timer);
  state.timer = enabled ? setInterval(() => loadPulse(), 30_000) : null;
}

function renderPulse(pulse) {
  renderMarkets(pulse.markets ?? []);
  renderTokenRows(ids.memeFlow, pulse.memeFlow ?? [], "No meme-flow tokens available.", "meme-count");
  renderTokenRows(ids.freshListings, pulse.freshListings ?? [], "No fresh listings available.", "listing-count");
  renderTokenRows(ids.newPairs, pulse.newPairs ?? [], "No new pairs available.", "pair-count");
  renderTradeRows(ids.recentSwaps, pulse.recentSwaps ?? [], "No recent swaps available.", "swap-count");
  renderTradeRows(ids.largeTrades, pulse.largeTrades ?? [], "No large trades available.", "large-count");
  if (pulse.errors?.length) {
    ids.analysisOutput.innerHTML = `<div class="notice warn">${escapeHtml(pulse.errors.map((item) => `${item.source}: ${item.error}`).join("\n"))}</div>`;
  }
}

function renderMarkets(markets) {
  ids.marketStrip.innerHTML = markets.map((market) => `
    <article class="metric-panel clickable" data-address="${escapeHtml(market.address)}">
      <span>${escapeHtml(market.label)}</span>
      <strong>${formatUsd(market.value)}</strong>
      <small>${market.updateUnixTime ? `Updated ${formatUnix(market.updateUnixTime)}` : "No timestamp"}</small>
    </article>
  `).join("");
  ids.marketStrip.querySelectorAll("[data-address]").forEach((row) => row.addEventListener("click", () => analyzeToken(row.dataset.address)));
}

function renderTokenRows(container, tokens, empty, countId) {
  document.getElementById(countId).textContent = tokens.length ? String(tokens.length) : "";
  if (!tokens.length) {
    container.innerHTML = `<div class="empty">${escapeHtml(empty)}</div>`;
    return;
  }
  container.innerHTML = tokens.map((token) => `
    <button class="data-row token-row" type="button" data-address="${escapeHtml(token.address)}">
      <span class="token-main">
        ${token.logoURI ? `<img src="${escapeHtml(token.logoURI)}" alt="" loading="lazy" />` : `<span class="token-dot"></span>`}
        <span><strong>${escapeHtml(token.symbol || shortAddress(token.address))}</strong><small>${escapeHtml(token.name || shortAddress(token.address))}</small></span>
      </span>
      <span>${formatUsd(token.price)}</span>
      <span>${formatUsd(token.liquidity)}</span>
      <span class="${Number(token.priceChange24hPercent) >= 0 ? "positive" : "negative"}">${formatPercent(token.priceChange24hPercent)}</span>
    </button>
  `).join("");
  container.querySelectorAll("[data-address]").forEach((row) => row.addEventListener("click", () => analyzeToken(row.dataset.address)));
}

function renderTradeRows(container, trades, empty, countId) {
  document.getElementById(countId).textContent = trades.length ? String(trades.length) : "";
  if (!trades.length) {
    container.innerHTML = `<div class="empty">${escapeHtml(empty)}</div>`;
    return;
  }
  container.innerHTML = trades.map((trade) => `
    <button class="data-row trade-row" type="button" data-address="${escapeHtml(trade.tokenAddress || "")}">
      <span><strong>${escapeHtml(trade.baseSymbol || trade.side || "swap")}</strong><small>${trade.source ? escapeHtml(trade.source) : shortAddress(trade.signature)}</small></span>
      <span>${formatUsd(trade.volumeUsd)}</span>
      <span>${formatNumber(trade.amount)}</span>
      <span>${trade.time ? formatUnix(trade.time) : "live"}</span>
    </button>
  `).join("");
  container.querySelectorAll("[data-address]").forEach((row) => {
    row.addEventListener("click", () => row.dataset.address ? analyzeToken(row.dataset.address) : null);
  });
}

async function analyzeToken(address) {
  const mint = String(address ?? "").trim();
  if (!isMint(mint)) {
    ids.analysisOutput.innerHTML = `<div class="notice warn">Enter or click a valid Solana token mint.</div>`;
    return;
  }
  state.selectedAddress = mint;
  ids.analysisAddress.value = mint;
  ids.analysisState.textContent = "Loading";
  ids.birdeyeLink.href = `https://birdeye.so/token/${mint}?chain=solana`;
  ids.jupiterLink.href = `https://jup.ag/swap/SOL-${mint}`;
  ids.analysisOutput.innerHTML = `<div class="empty">Loading Birdeye analysis...</div>`;
  try {
    const analysis = await getJson(`/api/swap/analyze?address=${encodeURIComponent(mint)}`);
    ids.analysisState.textContent = analysis.cached ? "Cached" : "Live";
    renderAnalysis(analysis);
  } catch (error) {
    ids.analysisState.textContent = "Error";
    ids.analysisOutput.innerHTML = `<div class="notice error">${escapeHtml(error.message)}</div>`;
  }
}

function renderAnalysis(analysis) {
  const token = analysis.profile ?? {};
  const risk = analysis.risk ?? {};
  ids.analysisOutput.innerHTML = `
    <div class="analysis-head">
      <div>
        <strong>${escapeHtml(token.symbol || shortAddress(analysis.address))}</strong>
        <span>${escapeHtml(token.name || analysis.address)}</span>
      </div>
      <span class="risk ${escapeHtml(risk.posture || "watch")}">${escapeHtml(risk.posture || "watch")}</span>
    </div>
    <div class="metric-grid">
      <span><small>Price</small><strong>${formatUsd(token.price)}</strong></span>
      <span><small>Liquidity</small><strong>${formatUsd(token.liquidity)}</strong></span>
      <span><small>24h volume</small><strong>${formatUsd(token.volume24hUSD)}</strong></span>
      <span><small>24h</small><strong class="${Number(token.priceChange24hPercent) >= 0 ? "positive" : "negative"}">${formatPercent(token.priceChange24hPercent)}</strong></span>
    </div>
    <div class="notice ${analysis.ok ? "ok" : "warn"}">${analysis.errors?.length ? escapeHtml(analysis.errors.map((item) => `${item.source}: ${item.error}`).join("\n")) : "Birdeye analysis loaded."}</div>
    <div class="flags">${(risk.flags ?? []).map((flag) => `<span>${escapeHtml(flag)}</span>`).join("") || "<span>no-flags</span>"}</div>
    <pre>${escapeHtml(JSON.stringify({ tradeData: analysis.tradeData, largeTrades: analysis.largeTrades?.slice(0, 5) }, null, 2))}</pre>
  `;
}

async function quoteSwap(event) {
  event.preventDefault();
  ids.quoteResult.textContent = "Quoting...";
  try {
    const quote = await postJson("/api/swap/quote", formData(event.currentTarget));
    const out = quote.quote?.outAmount;
    const impact = quote.quote?.priceImpactPct;
    const route = quote.routePlan?.map((item) => item.label).filter(Boolean).join(" / ") || "route returned";
    ids.quoteResult.innerHTML = `
      <strong>${out ? formatNumber(out) : "Quote ready"}</strong>
      <span>Impact ${formatPercent(Number(impact) * 100)} / ${escapeHtml(route)}</span>
    `;
  } catch (error) {
    ids.quoteResult.innerHTML = `<span class="negative">${escapeHtml(error.message)}</span>`;
  }
}

function setPair(pair) {
  document.getElementById("input-mint").value = pair === "jup-usdc" ? JUP_MINT : SOL_MINT;
  document.getElementById("output-mint").value = USDC_MINT;
  document.querySelector("#swap-form [name='decimals']").value = pair === "jup-usdc" ? "6" : "9";
}

function flipPair() {
  const input = document.getElementById("input-mint");
  const output = document.getElementById("output-mint");
  [input.value, output.value] = [output.value, input.value];
}

async function copySelectedAddress() {
  if (!state.selectedAddress) return;
  await navigator.clipboard.writeText(state.selectedAddress);
  ids.analysisState.textContent = "Copied";
}

function renderError(message) {
  [ids.memeFlow, ids.freshListings, ids.newPairs, ids.recentSwaps, ids.largeTrades].forEach((node) => {
    node.innerHTML = `<div class="notice error">${escapeHtml(message)}</div>`;
  });
}

function setStatus(text, mode) {
  ids.status.textContent = text;
  ids.status.dataset.mode = mode;
}

async function getJson(path) {
  const res = await fetch(path);
  const json = await res.json();
  if (!res.ok) throw new Error(json.error ?? `${path} ${res.status}`);
  return json;
}

async function postJson(path, body) {
  const res = await fetch(path, {
    method: "POST",
    headers: { "content-type": "application/json" },
    body: JSON.stringify(body),
  });
  const json = await res.json();
  if (!res.ok) throw new Error(json.error ?? `${path} ${res.status}`);
  return json;
}

function formData(form) {
  const data = new FormData(form);
  const out = {};
  for (const [key, value] of data.entries()) out[key] = value;
  return out;
}

function isMint(value) {
  return /^[1-9A-HJ-NP-Za-km-z]{32,64}$/.test(String(value ?? "").trim());
}

function formatUsd(value) {
  const number = Number(value);
  if (!Number.isFinite(number)) return "n/a";
  if (Math.abs(number) < 0.01) return `$${number.toPrecision(4)}`;
  return new Intl.NumberFormat("en-US", { style: "currency", currency: "USD", maximumFractionDigits: number > 100 ? 2 : 6 }).format(number);
}

function formatPercent(value) {
  const number = Number(value);
  if (!Number.isFinite(number)) return "n/a";
  return `${number >= 0 ? "+" : ""}${number.toFixed(2)}%`;
}

function formatNumber(value) {
  const number = Number(value);
  if (!Number.isFinite(number)) return "n/a";
  return new Intl.NumberFormat("en-US", { maximumFractionDigits: 4 }).format(number);
}

function formatUnix(value) {
  const date = new Date(Number(value) * 1000);
  return Number.isNaN(date.getTime()) ? "n/a" : date.toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" });
}

function formatAge(value) {
  const ms = Date.now() - new Date(value).getTime();
  if (!Number.isFinite(ms)) return "fresh";
  return `${Math.max(0, Math.round(ms / 1000))}s ago`;
}

function shortAddress(address) {
  return address ? `${address.slice(0, 4)}...${address.slice(-4)}` : "n/a";
}

function escapeHtml(value) {
  return String(value ?? "")
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;")
    .replaceAll('"', "&quot;")
    .replaceAll("'", "&#039;");
}
