#!/usr/bin/env node
import { createServer } from "node:http";
import { createHash } from "node:crypto";
import { existsSync, readFileSync } from "node:fs";
import { extname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { dirname } from "node:path";

const __dirname = dirname(fileURLToPath(import.meta.url));
const repoRoot = findRepoRoot(__dirname);
const publicRoot = join(__dirname, "public");
const registryPath = resolve(repoRoot, "data/ptokens.json");

const PORT = Number(process.env.PORT ?? 8787);
const DEFAULT_RPC = process.env.SOLANA_RPC_URL ?? process.env.HELIUS_RPC_URL ?? "https://api.devnet.solana.com";
const DEFAULT_P_TOKEN_PROGRAM_ID = process.env.P_TOKEN_PROGRAM_ID ?? "ptok6rngomXrDbWf5v5Mkmu5CEbB51hzSCPDoj9DrvF";
const SPL_TOKEN_PROGRAM_ID = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
const BIRDEYE_API_BASE = process.env.BIRDEYE_API_BASE ?? "https://public-api.birdeye.so";
const BIRDEYE_API_KEY = process.env.BIRDEYE_API_KEY ?? process.env.BIRDEYE_KEY ?? "";
const JUPITER_QUOTE_URL = process.env.JUPITER_QUOTE_URL ?? "https://api.jup.ag/swap/v1/quote";
const SOL_MINT = "So11111111111111111111111111111111111111112";
const USDC_MINT = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";
const JUP_MINT = "JUPyiwrYJFskUPiHa7hkeR8VUtAeFoSYbKedZNsDvCN";
const DEX_CACHE_TTL_MS = Number(process.env.DEX_CACHE_TTL_MS ?? 15_000);
const dexCache = new Map();

const mimeTypes = {
  ".html": "text/html; charset=utf-8",
  ".js": "text/javascript; charset=utf-8",
  ".css": "text/css; charset=utf-8",
  ".json": "application/json; charset=utf-8",
  ".svg": "image/svg+xml",
  ".png": "image/png",
};

const server = createServer(async (req, res) => {
  try {
    const url = new URL(req.url ?? "/", `http://${req.headers.host ?? "localhost"}`);
    if (url.pathname.startsWith("/api/")) {
      await handleApi(req, res, url);
      return;
    }
    serveStatic(res, url.pathname);
  } catch (error) {
    sendJson(res, 500, { error: error instanceof Error ? error.message : String(error) });
  }
});

server.listen(PORT, () => {
  console.log(`p-token launcher workbench: http://localhost:${PORT}`);
});

function findRepoRoot(start) {
  let current = resolve(start);
  while (true) {
    if (
      existsSync(join(current, "package.json")) &&
      (existsSync(join(current, "shared")) || existsSync(join(current, "templates")))
    ) {
      return current;
    }

    const parent = resolve(current, "..");
    if (parent === current) return resolve(start, "..");
    current = parent;
  }
}

async function handleApi(req, res, url) {
  if (req.method === "GET" && url.pathname === "/api/health") {
    sendJson(res, 200, {
      ok: true,
      unsigned: true,
      network: inferNetwork(DEFAULT_RPC),
      rpcConfigured: Boolean(process.env.SOLANA_RPC_URL || process.env.HELIUS_RPC_URL),
      pTokenProgramId: DEFAULT_P_TOKEN_PROGRAM_ID,
      launchpadProgramId: process.env.P_TOKEN_LAUNCHPAD_PROGRAM_ID ?? "11111111111111111111111111111111",
      agentProgramId: process.env.P_AGENT_TOKEN_PROGRAM_ID ?? "11111111111111111111111111111111",
      adapters: ["wdk-intent", "browser-wallet", "agent-review", "human-review"],
    });
    return;
  }

  if (req.method === "GET" && url.pathname === "/api/workspace") {
    sendJson(res, 200, workspaceMap());
    return;
  }

  if (req.method === "GET" && url.pathname === "/api/pinocchio") {
    sendJson(res, 200, pinocchioMap());
    return;
  }

  if (req.method === "GET" && url.pathname === "/api/registry") {
    sendJson(res, 200, readRegistry());
    return;
  }

  if (req.method === "GET" && url.pathname === "/api/examples") {
    sendJson(res, 200, {
      launchConfig: readJson(join(__dirname, "launch-config.example.json")),
      bondingCurve: readJson(join(__dirname, "bonding-curve.example.json")),
    });
    return;
  }

  if (req.method === "GET" && url.pathname === "/api/swap/pulse") {
    sendJson(res, 200, await dexPulse());
    return;
  }

  if (req.method === "GET" && url.pathname === "/api/swap/analyze") {
    sendJson(res, 200, await tokenAnalysis(String(url.searchParams.get("address") ?? "")));
    return;
  }

  if (req.method === "POST" && url.pathname === "/api/swap/quote") {
    sendJson(res, 200, await swapQuote(await readJsonBody(req)));
    return;
  }

  if (req.method === "POST" && url.pathname === "/api/launch-plan") {
    sendJson(res, 200, launchPlan(await readJsonBody(req)));
    return;
  }

  if (req.method === "POST" && url.pathname === "/api/agent-plan") {
    sendJson(res, 200, agentPlan(await readJsonBody(req)));
    return;
  }

  if (req.method === "POST" && url.pathname === "/api/program-draft") {
    sendJson(res, 200, programDraft(await readJsonBody(req)));
    return;
  }

  if (req.method === "POST" && url.pathname === "/api/explore") {
    sendJson(res, 200, explore(await readJsonBody(req)));
    return;
  }

  if (req.method === "POST" && url.pathname === "/api/quote") {
    sendJson(res, 200, quote(await readJsonBody(req)));
    return;
  }

  if (req.method === "POST" && url.pathname === "/api/rise-floor") {
    sendJson(res, 200, riseFloor(await readJsonBody(req)));
    return;
  }

  if (req.method === "POST" && url.pathname === "/api/perp-plan") {
    sendJson(res, 200, perpPlan(await readJsonBody(req)));
    return;
  }

  if (req.method === "POST" && url.pathname === "/api/wdk-intent") {
    sendJson(res, 200, wdkIntent(await readJsonBody(req)));
    return;
  }

  if (req.method === "POST" && url.pathname === "/api/inspect") {
    const body = await readJsonBody(req);
    sendJson(res, 200, await inspectMint(String(body.mint ?? ""), body));
    return;
  }

  sendJson(res, 404, { error: `No route for ${req.method} ${url.pathname}` });
}

function serveStatic(res, pathname) {
  const normalized = pathname === "/" ? "/index.html" : pathname === "/swap" ? "/swap.html" : pathname;
  const path = resolve(publicRoot, `.${normalized}`);
  if (!path.startsWith(publicRoot) || !existsSync(path)) {
    sendJson(res, 404, { error: "Not found" });
    return;
  }
  res.writeHead(200, { "content-type": mimeTypes[extname(path)] ?? "application/octet-stream" });
  res.end(readFileSync(path));
}

async function readJsonBody(req) {
  const chunks = [];
  for await (const chunk of req) chunks.push(chunk);
  if (chunks.length === 0) return {};
  return JSON.parse(Buffer.concat(chunks).toString("utf8"));
}

function sendJson(res, status, payload) {
  res.writeHead(status, {
    "content-type": "application/json; charset=utf-8",
    "cache-control": "no-store",
  });
  res.end(JSON.stringify(payload, null, 2));
}

async function cachedJson(key, loader, ttlMs = DEX_CACHE_TTL_MS) {
  const cached = dexCache.get(key);
  if (cached && Date.now() - cached.time < ttlMs) return { ...cached.value, cached: true };
  const value = await loader();
  dexCache.set(key, { time: Date.now(), value });
  return { ...value, cached: false };
}

async function dexPulse() {
  return cachedJson("dex-pulse", async () => {
    const generatedAt = new Date().toISOString();
    const calls = await Promise.allSettled([
      birdeye("price:sol", "/defi/price", { address: SOL_MINT }),
      birdeye("price:jup", "/defi/price", { address: JUP_MINT }),
      birdeye("trending", "/defi/token_trending", { sort_by: "rank", sort_type: "asc", offset: 0, limit: 12 }),
      birdeye("new-listing", "/defi/v2/tokens/new_listing", { limit: 12, meme_platform_enabled: true }),
      birdeye("recent", "/defi/v3/txs/recent", { limit: 16, tx_type: "swap" }),
      birdeye("large", "/defi/v3/token/txs-by-volume", {
        token_address: SOL_MINT,
        volume_type: "usd",
        min_volume: 25_000,
        tx_type: "swap",
        sort_by: "block_unix_time",
        sort_type: "desc",
        offset: 0,
        limit: 12,
      }),
    ]);
    const [solPrice, jupPrice, trending, newListings, recentSwaps, largeTrades] = calls.map(settledPayload);
    const trendingTokens = normalizeTokens(trending.data);
    const listingTokens = normalizeTokens(newListings.data);
    const recentRows = normalizeTrades(recentSwaps.data);
    const largeRows = normalizeTrades(largeTrades.data);
    const errors = calls
      .map((call, index) => call.status === "rejected" ? { source: ["SOL/USD", "JUP/USD", "trending", "new-listing", "recent-swaps", "large-trades"][index], error: call.reason.message } : null)
      .filter(Boolean);

    return {
      ok: errors.length === 0,
      generatedAt,
      provider: {
        birdeye: {
          configured: Boolean(BIRDEYE_API_KEY),
          baseUrl: BIRDEYE_API_BASE,
          status: BIRDEYE_API_KEY ? "live" : "missing-api-key",
        },
        jupiter: {
          quoteUrl: JUPITER_QUOTE_URL,
        },
      },
      markets: [
        priceCard("SOL/USD", SOL_MINT, solPrice.data),
        priceCard("JUP/USD", JUP_MINT, jupPrice.data),
      ],
      memeFlow: buildMemeFlow(trendingTokens, listingTokens),
      freshListings: listingTokens,
      newPairs: listingTokens.filter((token) => token.liquidity > 0 || token.source).slice(0, 12),
      recentSwaps: recentRows,
      largeTrades: largeRows,
      defaults: {
        inputMint: SOL_MINT,
        outputMint: USDC_MINT,
        slippageBps: 75,
      },
      errors,
    };
  });
}

async function tokenAnalysis(address) {
  const mint = sanitizeMint(address);
  return cachedJson(`analysis:${mint}`, async () => {
    const calls = await Promise.allSettled([
      birdeye(`overview:${mint}`, "/defi/token_overview", { address: mint, frames: "1h,4h,24h" }),
      birdeye(`trade-data:${mint}`, "/defi/v3/token/trade-data/single", { address: mint, frames: "1h,4h,24h" }),
      birdeye(`txs:${mint}`, "/defi/v3/token/txs", { address: mint, limit: 20, tx_type: "swap", sort_type: "desc" }),
      birdeye(`large:${mint}`, "/defi/v3/token/txs-by-volume", {
        token_address: mint,
        volume_type: "usd",
        min_volume: 5_000,
        tx_type: "swap",
        sort_by: "block_unix_time",
        sort_type: "desc",
        offset: 0,
        limit: 12,
      }),
    ]);
    const [overview, tradeData, txs, large] = calls.map(settledPayload);
    const profile = normalizeToken(overview.data?.data ?? overview.data, { address: mint });
    const recentSwaps = normalizeTrades(txs.data);
    const largeTrades = normalizeTrades(large.data);
    const risk = riskSnapshot(profile, recentSwaps, largeTrades);
    return {
      ok: calls.every((call) => call.status === "fulfilled"),
      generatedAt: new Date().toISOString(),
      address: mint,
      profile,
      tradeData: tradeData.data?.data ?? tradeData.data ?? null,
      recentSwaps,
      largeTrades,
      risk,
      errors: calls
        .map((call, index) => call.status === "rejected" ? { source: ["overview", "trade-data", "recent-swaps", "large-trades"][index], error: call.reason.message } : null)
        .filter(Boolean),
    };
  });
}

async function swapQuote(input) {
  const inputMint = sanitizeMint(input.inputMint ?? SOL_MINT);
  const outputMint = sanitizeMint(input.outputMint ?? USDC_MINT);
  if (inputMint === outputMint) throw new Error("inputMint and outputMint must be different");
  const humanAmount = Number(input.amount ?? 1);
  if (!Number.isFinite(humanAmount) || humanAmount <= 0) throw new Error("amount must be greater than zero");
  const decimals = Math.max(0, Math.min(12, Number(input.decimals ?? (inputMint === SOL_MINT ? 9 : 6))));
  const amount = toBaseUnits(humanAmount, decimals);
  const slippageBps = Math.max(1, Math.min(5000, Number(input.slippageBps ?? 75)));
  const params = new URLSearchParams({
    inputMint,
    outputMint,
    amount,
    slippageBps: String(slippageBps),
    swapMode: "ExactIn",
  });
  const res = await fetch(`${JUPITER_QUOTE_URL}?${params.toString()}`, {
    headers: { accept: "application/json" },
    signal: AbortSignal.timeout(8_000),
  });
  const json = await safeJson(res);
  if (!res.ok) throw new Error(`Jupiter quote HTTP ${res.status}: ${extractError(json)}`);
  return {
    ok: true,
    generatedAt: new Date().toISOString(),
    unsigned: true,
    inputMint,
    outputMint,
    amount,
    humanAmount,
    decimals,
    slippageBps,
    quote: json,
    routePlan: Array.isArray(json.routePlan) ? json.routePlan.map((route) => ({
      percent: route.percent,
      label: route.swapInfo?.label,
      ammKey: route.swapInfo?.ammKey,
      inputMint: route.swapInfo?.inputMint,
      outputMint: route.swapInfo?.outputMint,
      inAmount: route.swapInfo?.inAmount,
      outAmount: route.swapInfo?.outAmount,
      feeAmount: route.swapInfo?.feeAmount,
      feeMint: route.swapInfo?.feeMint,
    })) : [],
  };
}

async function birdeye(key, path, params) {
  if (!BIRDEYE_API_KEY) throw new Error("BIRDEYE_API_KEY is not configured");
  const url = new URL(path, BIRDEYE_API_BASE);
  for (const [param, value] of Object.entries(params ?? {})) {
    if (value !== undefined && value !== null && value !== "") url.searchParams.set(param, String(value));
  }
  const res = await fetch(url, {
    headers: {
      accept: "application/json",
      "x-chain": "solana",
      "X-API-KEY": BIRDEYE_API_KEY,
    },
    signal: AbortSignal.timeout(8_000),
  });
  const json = await safeJson(res);
  if (!res.ok) throw new Error(`${key} HTTP ${res.status}: ${extractError(json)}`);
  if (json && json.success === false) throw new Error(`${key}: ${extractError(json)}`);
  return json;
}

async function safeJson(res) {
  const text = await res.text();
  if (!text) return null;
  try {
    return JSON.parse(text);
  } catch {
    return { raw: text.slice(0, 500) };
  }
}

function settledPayload(result) {
  return result.status === "fulfilled" ? { data: result.value } : { data: null, error: result.reason.message };
}

function extractError(json) {
  return json?.message ?? json?.error ?? json?.data?.message ?? json?.raw ?? "request failed";
}

function sanitizeMint(value) {
  const mint = String(value ?? "").trim();
  if (!/^[1-9A-HJ-NP-Za-km-z]{32,64}$/.test(mint)) throw new Error(`Invalid Solana mint: ${mint || "<empty>"}`);
  return mint;
}

function toBaseUnits(amount, decimals) {
  const fixed = amount.toFixed(decimals);
  const [whole, fraction = ""] = fixed.split(".");
  return `${whole}${fraction.padEnd(decimals, "0")}`.replace(/^0+(?=\d)/, "") || "0";
}

function priceCard(label, address, payload) {
  const data = payload?.data ?? payload;
  return {
    label,
    address,
    value: numberOrNull(data?.value ?? data?.price),
    updateUnixTime: data?.updateUnixTime ?? data?.update_unix_time ?? null,
  };
}

function normalizeTokens(payload) {
  const data = payload?.data ?? payload;
  const list = data?.tokens ?? data?.items ?? data?.list ?? data?.data ?? [];
  return Array.isArray(list) ? list.map((item) => normalizeToken(item)).filter((item) => item.address).slice(0, 24) : [];
}

function normalizeToken(item, fallback = {}) {
  const address = item?.address ?? item?.tokenAddress ?? item?.mint ?? fallback.address ?? "";
  return {
    address,
    symbol: item?.symbol ?? item?.tokenSymbol ?? fallback.symbol ?? shortAddress(address),
    name: item?.name ?? item?.tokenName ?? fallback.name ?? shortAddress(address),
    logoURI: item?.logoURI ?? item?.logo_uri ?? item?.logo ?? null,
    price: numberOrNull(item?.price ?? item?.value),
    liquidity: numberOrNull(item?.liquidity ?? item?.liquidityUsd ?? item?.liquidityUSD),
    volume24hUSD: numberOrNull(item?.volume24hUSD ?? item?.v24hUSD ?? item?.volumeUSD ?? item?.volume_usd),
    priceChange24hPercent: numberOrNull(item?.priceChange24hPercent ?? item?.v24hChangePercent ?? item?.priceChange24h),
    marketCap: numberOrNull(item?.mc ?? item?.marketCap ?? item?.marketcap),
    source: item?.source ?? item?.sourceName ?? item?.platform ?? null,
    rank: item?.rank ?? null,
    listedAt: item?.liquidityAddedAt ?? item?.listedAt ?? item?.createdAt ?? item?.blockUnixTime ?? null,
  };
}

function normalizeTrades(payload) {
  const data = payload?.data ?? payload;
  const list = data?.items ?? data?.txs ?? data?.transactions ?? data?.list ?? data?.data ?? [];
  return Array.isArray(list) ? list.map((item) => ({
    signature: item.txHash ?? item.tx_hash ?? item.signature ?? item.txSignature ?? null,
    side: item.side ?? item.txType ?? item.tx_type ?? item.type ?? "swap",
    source: item.source ?? item.sourceName ?? item.dex ?? item.platform ?? null,
    tokenAddress: item.tokenAddress ?? item.token_address ?? item.address ?? item.baseAddress ?? null,
    baseSymbol: item.baseSymbol ?? item.tokenSymbol ?? item.symbol ?? item.from?.symbol ?? null,
    quoteSymbol: item.quoteSymbol ?? item.quote?.symbol ?? item.to?.symbol ?? null,
    volumeUsd: numberOrNull(item.volumeUSD ?? item.volume_usd ?? item.valueUSD ?? item.value_usd ?? item.usd),
    amount: numberOrNull(item.amount ?? item.uiAmount ?? item.tokenAmount),
    price: numberOrNull(item.price ?? item.priceUsd ?? item.price_usd),
    owner: item.owner ?? item.trader ?? item.maker ?? null,
    time: item.blockUnixTime ?? item.block_unix_time ?? item.unixTime ?? item.time ?? null,
  })).slice(0, 32) : [];
}

function buildMemeFlow(trendingTokens, listingTokens) {
  const combined = [...listingTokens, ...trendingTokens];
  return combined
    .filter((token) => token.address && (token.address.endsWith("pump") || String(token.source ?? "").toLowerCase().includes("pump") || String(token.name ?? "").toLowerCase().includes("pump")))
    .slice(0, 12);
}

function riskSnapshot(profile, recentSwaps, largeTrades) {
  const liquidity = Number(profile.liquidity ?? 0);
  const volume = Number(profile.volume24hUSD ?? 0);
  const largeVolume = largeTrades.reduce((sum, trade) => sum + Number(trade.volumeUsd ?? 0), 0);
  const flags = [];
  if (!liquidity) flags.push("missing-liquidity");
  if (liquidity > 0 && volume / liquidity > 10) flags.push("high-volume-to-liquidity");
  if (largeVolume > volume * 0.5 && volume > 0) flags.push("large-trade-concentration");
  if (recentSwaps.length === 0) flags.push("no-recent-swaps");
  return {
    liquidity,
    volume24hUSD: volume,
    largeTradeVolumeUSD: largeVolume,
    flags,
    posture: flags.length === 0 ? "normal" : flags.length <= 2 ? "watch" : "high-risk",
  };
}

function numberOrNull(value) {
  const number = Number(value);
  return Number.isFinite(number) ? number : null;
}

function shortAddress(address) {
  return address ? `${address.slice(0, 4)}...${address.slice(-4)}` : "";
}

function readJson(path) {
  return JSON.parse(readFileSync(path, "utf8"));
}

function readRegistry() {
  if (!existsSync(registryPath)) return { version: 1, tokens: [] };
  const parsed = readJson(registryPath);
  return { version: parsed.version ?? 1, tokens: Array.isArray(parsed.tokens) ? parsed.tokens : [] };
}

function workspaceMap() {
  return {
    repoRoot,
    unsigned: true,
    pinocchio: pinocchioMap(),
    packages: [
      {
        name: "agent-sdk",
        path: "agent-sdk",
        role: "Core asset identity, asset signer PDA, registration docs, and Core Execute wrappers.",
        entrypoints: ["src/p-agent.ts", "src/p-nft.ts", "src/p-registry.ts", "src/client.ts"],
      },
      {
        name: "launchpad",
        path: "launchpad",
        role: "Bonding curve math, launch/trade instruction builders, state reader, and x402-gated API.",
        entrypoints: ["src/server.ts", "src/programs/launchpad-ix.ts", "src/curves/constant-product.ts"],
      },
      {
        name: "p-agent-token",
        path: "p-agent-token",
        role: "Pinocchio agent-token program draft for devnet and mainnet deployments.",
        entrypoints: ["src/lib.rs", "src/state.rs", "src/instructions"],
      },
      {
        name: "facilitator",
        path: "facilitator",
        role: "x402 verify, settle, supported, and health service.",
        entrypoints: ["src/server.ts"],
      },
      {
        name: "perps",
        path: "perps",
        role: "Drift and Adrena-style market configuration for graduated p-tokens.",
        entrypoints: ["src/drift-adapter.ts"],
      },
    ],
    documents: [
      "docs/PROTOCOL.md",
      "docs/P_AGENTS.md",
      "docs/PROGRAM_DRAFT.md",
      "docs/PINOCCHIO_ADAPTATION.md",
      "p-agent-token/DEPLOYMENT.md",
    ],
    templates: ["templates/escrow", "templates/vault", "templates/p-agent-token", "templates/p-token-launcher"],
  };
}

function pinocchioMap() {
  return {
    source: "pinocchio/pinocchio-main",
    sdk: {
      crate: "pinocchio",
      path: "pinocchio/pinocchio-main/sdk",
      version: "0.11.1",
      exports: ["AccountView", "Address", "ProgramResult", "error::ProgramError", "entrypoint"],
    },
    helperPrograms: [
      {
        crate: "pinocchio-system",
        path: "pinocchio/pinocchio-main/programs/system",
        usedBy: ["p-agent-token", "programs/src"],
      },
      {
        crate: "pinocchio-token",
        path: "pinocchio/pinocchio-main/programs/token",
        usedBy: ["p-agent-token", "programs/src"],
      },
      {
        crate: "pinocchio-associated-token-account",
        path: "pinocchio/pinocchio-main/programs/associated-token-account",
        usedBy: ["p-agent-token"],
      },
      {
        crate: "pinocchio-token-2022",
        path: "pinocchio/pinocchio-main/programs/token-2022",
        usedBy: ["future token extension work"],
      },
      {
        crate: "pinocchio-memo",
        path: "pinocchio/pinocchio-main/programs/memo",
        usedBy: ["future audit trail memos"],
      },
    ],
    adaptedSurfaces: [
      {
        surface: "p-agent-token",
        status: "path-linked and cargo-checking",
        contract: "Core asset binding, agent-token state, execution delegation",
      },
      {
        surface: "programs/src",
        status: "path-linked and cargo-checking",
        contract: "Bonding curve launchpad program shell",
      },
      {
        surface: "p-token-launcher",
        status: "exposes Pinocchio map and deployment draft endpoints",
        contract: "Frontend/API exploration of p-token and P Agent program plans",
      },
      {
        surface: "launchpad",
        status: "TypeScript builders mirror Pinocchio instruction discriminators",
        contract: "Unsigned client instructions and x402-gated HTTP routes",
      },
    ],
    checks: [
      "CARGO_TARGET_DIR=/tmp/letterp-p-agent-token-target cargo check",
      "CARGO_TARGET_DIR=/tmp/letterp-bonding-curve-target cargo check",
    ],
  };
}

function numberArg(input, key, fallback) {
  const raw = input[key];
  if (raw === undefined || raw === null || raw === "") return fallback;
  const value = Number(raw);
  if (!Number.isFinite(value) || value < 0) throw new Error(`Invalid ${key}: ${raw}`);
  return value;
}

function tokenAmount(humanAmount, decimals) {
  const [whole, frac = ""] = String(humanAmount).split(".");
  const padded = `${frac}${"0".repeat(decimals)}`.slice(0, decimals);
  return `${whole}${padded}`.replace(/^0+(?=\d)/, "") || "0";
}

function launchPlan(input) {
  const symbol = String(input.symbol ?? "PQC").trim().toUpperCase();
  const name = String(input.name ?? "Quantum Compute p-token").trim();
  const decimals = numberArg(input, "decimals", 9);
  const supply = numberArg(input, "supply", 1_000_000_000);
  const virtualSol = numberArg(input, "virtualSol", 30);
  const virtualToken = numberArg(input, "virtualToken", 1_073_000_000);
  const realSol = numberArg(input, "realSol", 0);
  const realToken = numberArg(input, "realToken", 793_100_000);
  const feeBps = numberArg(input, "feeBps", 100);
  const floor = riseFloor({ ...input, virtualSol, virtualToken, feeBps });
  return {
    unsigned: true,
    warning: "Planning artifact only. No deploy, signing, mint creation, transfers, borrows, or trades are executed.",
    network: String(input.network ?? "solana-devnet"),
    audience: ["agents", "humans", "quantum-compute operators"],
    tokenProgram: "p-token",
    pTokenProgramId: String(input.pTokenProgramId ?? DEFAULT_P_TOKEN_PROGRAM_ID),
    metadata: {
      name,
      symbol,
      uri: String(input.uri ?? "https://example.com/pqc.json"),
      decimals,
      thesis: String(input.thesis ?? "p-token launchpad for compute, agents, and human trading coordination."),
    },
    supply: {
      human: supply,
      baseUnits: tokenAmount(supply, decimals),
    },
    authorities: {
      mintAuthority: String(input.mintAuthority ?? "<wallet-or-pda>"),
      freezeAuthority: input.freezeAuthority ? String(input.freezeAuthority) : null,
      updateAuthority: String(input.updateAuthority ?? "<wallet-or-pda>"),
    },
    bondingCurve: {
      enabled: input.bondingCurve !== false,
      type: "constant-product",
      virtualSol,
      virtualToken,
      realSol,
      realToken,
      feeBps,
      spotPrice: virtualSol / virtualToken,
      graduation: {
        trigger: "real-sol-reserve",
        targetSol: numberArg(input, "graduationSol", 85),
        postGraduation: "seed-amm-liquidity",
      },
    },
    riseStyleFloor: floor,
    perpetuals: perpPlan(input),
    wdk: wdkIntent({ action: "launch-plan", ...input, symbol, name }),
    checklist: [
      "review p-token program id and target cluster",
      "lock PDA seeds, reserve custody, signer checks, and authority handoff in Pinocchio code",
      "simulate buy, sell, borrow, repay, loop, and graduation state transitions",
      "wire WDK/browser wallet signing only after human or policy approval",
      "inspect the mint over RPC before registry changes",
      "register verified mint in data/ptokens.json",
      "enable x402 P_TOKEN_PROGRAM_ID or USE_P_TOKEN routing only after verification",
    ],
  };
}

function agentPlan(input) {
  const agentName = String(input.agentName ?? "Solana Clawd").trim();
  const tokenSymbol = String(input.symbol ?? "CLAWD").trim().toUpperCase();
  const owner = String(input.owner ?? "<owner-wallet-or-multisig>");
  const asset = String(input.coreAsset ?? "<new-metaplex-core-asset>");
  const model = String(input.model ?? "policy-controlled-agent");
  const endpoint = String(input.endpoint ?? "https://solanaclawd.com/agent.json");
  const assetSigner = draftAddress("mpl-core-execute", asset);
  const agentState = draftAddress("agent", assetSigner);
  const creatorVault = draftAddress("creator-vault", assetSigner);
  const executive = input.executive ? String(input.executive) : null;

  return {
    unsigned: true,
    standard: "p-agent-token-v1",
    network: String(input.network ?? "solana-devnet"),
    agent: {
      name: agentName,
      owner,
      coreAsset: asset,
      assetSigner,
      state: agentState,
      executive,
      endpoint,
      model,
      capabilities: normalizeList(input.capabilities ?? "launch,trade,quote,settle,delegate"),
    },
    token: {
      symbol: tokenSymbol,
      name: String(input.tokenName ?? `${agentName} Token`),
      mint: String(input.mint ?? "<created-by-launch>"),
      creatorFeeWallet: assetSigner,
      creatorVault,
    },
    registrationDoc: {
      "@context": "https://erc8004.org/schema/agent.json",
      "@type": "Agent",
      id: asset,
      name: agentName,
      description: String(input.description ?? `${agentName} manages p-token launch and trading workflows.`),
      image: String(input.image ?? "https://solanaclawd.com/agent.png"),
      model,
      capabilities: normalizeList(input.capabilities ?? "launch,trade,quote,settle,delegate"),
      endpoint,
      services: [
        { name: "launchpad", endpoint: String(input.launchpadUrl ?? "http://localhost:4400"), version: "0.1.0" },
        { name: "x402", endpoint: String(input.facilitatorUrl ?? "http://localhost:4402"), version: "0.1.0" },
      ],
      active: true,
      registrations: [{ agentId: asset, agentRegistry: "solana:p-agent-token-v1" }],
      supportedTrust: ["core-execute", "x402", "human-review"],
    },
    executionPolicy: {
      defaultMode: "review-required",
      delegateExpiresAtSlot: input.delegateExpiresAtSlot ? Number(input.delegateExpiresAtSlot) : null,
      blockedActionsUntilImplemented: ["custodial-transfer", "autonomous-mainnet-trade", "perp-open-without-policy"],
    },
    nextInstructions: [
      "mint or select the Core asset",
      "pin the registration JSON and call initialize_agent",
      "initialize the p-token mint and bind_agent_token",
      "route creator fees to the asset signer PDA",
      "delegate execution only with slot expiry and policy checks",
    ],
  };
}

function programDraft(input) {
  const target = String(input.target ?? "devnet");
  const authority = String(input.upgradeAuthority ?? "<upgrade-authority-multisig>");
  const launchpadProgramId = String(input.launchpadProgramId ?? process.env.P_TOKEN_LAUNCHPAD_PROGRAM_ID ?? "<deploy-launchpad-program>");
  const agentProgramId = String(input.agentProgramId ?? process.env.P_AGENT_TOKEN_PROGRAM_ID ?? "<deploy-agent-token-program>");
  const pTokenProgramId = String(input.pTokenProgramId ?? DEFAULT_P_TOKEN_PROGRAM_ID);
  const risk = target.includes("mainnet") ? "mainnet-guarded" : "devnet-iteration";
  return {
    unsigned: true,
    target,
    risk,
    programs: {
      launchpad: {
        path: "programs/src",
        programId: launchpadProgramId,
        purpose: "bonding curve state, buy/sell/graduation, creator fee claiming",
        sourceOfTruth: "docs/PROTOCOL.md",
        pinocchioDeps: ["pinocchio", "pinocchio-system", "pinocchio-token"],
      },
      agentToken: {
        path: "p-agent-token",
        programId: agentProgramId,
        purpose: "agent state, Core asset binding, p-token mint binding, executive delegation",
        sourceOfTruth: "p-agent-token/DEPLOYMENT.md",
        pinocchioDeps: ["pinocchio", "pinocchio-system", "pinocchio-token", "pinocchio-associated-token-account"],
      },
      tokenProgram: {
        programId: pTokenProgramId,
        purpose: "p-token or SPL-compatible token operations",
      },
    },
    deployment: {
      upgradeAuthority: authority,
      devnet: [
        "cargo check in p-agent-token",
        "cargo build-sbf or Solana SBF build for the selected Pinocchio toolchain",
        "solana config set --url devnet",
        "solana program deploy target/deploy/p_agent_token.so --program-id target/deploy/p_agent_token-keypair.json",
        "set P_AGENT_TOKEN_PROGRAM_ID to the deployed program id",
        "run launcher inspect and agent-plan against devnet",
      ],
      mainnet: [
        "freeze interface discriminators and account layouts",
        "run SBF tests and external review",
        "deploy with multisig upgrade authority",
        "publish program ids in docs and environment templates",
        "start with review-required agent execution policy",
      ],
    },
    accountContract: {
      agent: ["owner", "agent_asset", "agent_token_mint", "executive", "metadata_hash", "flags", "bump"],
      curve: ["mint", "vault", "virtual_sol", "virtual_token", "real_sol", "real_token", "fee_bps", "creator_fee_bps", "flags", "bump"],
      signerSeeds: {
        assetSigner: ["mpl-core-execute", "asset"],
        agentState: ["agent", "asset_signer"],
        agentToken: ["agent-token", "mint"],
        curve: ["bonding-curve", "mint"],
        vault: ["bonding-curve", "mint", "vault"],
      },
    },
  };
}

function explore(input) {
  const query = String(input.query ?? "").trim();
  const kind = String(input.kind ?? inferExploreKind(query));
  return {
    unsigned: true,
    query,
    kind,
    network: String(input.network ?? inferNetwork(DEFAULT_RPC)),
    token: query ? classifyTokenInput(query) : null,
    workspace: workspaceMap().packages,
    routes: {
      launchPlan: "/api/launch-plan",
      agentPlan: "/api/agent-plan",
      quote: "/api/quote",
      inspect: "/api/inspect",
      programDraft: "/api/program-draft",
    },
    recommendedNext: recommendedNext(kind),
  };
}

function quote(input) {
  const virtualSol = numberArg(input, "virtualSol", 30);
  const virtualToken = numberArg(input, "virtualToken", 1_073_000_000);
  const feeBps = numberArg(input, "feeBps", 100);
  if (input.side === "sell" || input.tokens !== undefined) {
    const tokensIn = numberArg(input, "tokens", 1_000_000);
    const k = virtualSol * virtualToken;
    const virtualTokenAfter = virtualToken + tokensIn;
    const virtualSolAfter = k / virtualTokenAfter;
    const grossSolOut = Math.max(0, virtualSol - virtualSolAfter);
    const fee = grossSolOut * feeBps / 10_000;
    return {
      unsigned: true,
      side: "sell",
      tokensIn,
      grossSolOut,
      fee,
      netSolOut: grossSolOut - fee,
      spotPriceBefore: virtualSol / virtualToken,
      spotPriceAfter: virtualSolAfter / virtualTokenAfter,
      virtualSolAfter,
      virtualTokenAfter,
    };
  }
  const solIn = numberArg(input, "sol", 1);
  const fee = solIn * feeBps / 10_000;
  const netSolIn = solIn - fee;
  const k = virtualSol * virtualToken;
  const virtualSolAfter = virtualSol + netSolIn;
  const virtualTokenAfter = k / virtualSolAfter;
  return {
    unsigned: true,
    side: "buy",
    solIn,
    fee,
    netSolIn,
    tokensOut: Math.max(0, virtualToken - virtualTokenAfter),
    spotPriceBefore: virtualSol / virtualToken,
    spotPriceAfter: virtualSolAfter / virtualTokenAfter,
    virtualSolAfter,
    virtualTokenAfter,
  };
}

function riseFloor(input) {
  const virtualSol = numberArg(input, "virtualSol", 30);
  const virtualToken = numberArg(input, "virtualToken", 1_073_000_000);
  const athPrice = numberArg(input, "athPrice", virtualSol / virtualToken);
  const floorRatio = Math.min(numberArg(input, "floorRatio", 0.5), 0.95);
  const protocolSol = numberArg(input, "protocolSol", numberArg(input, "realSol", 0));
  const circulatingTokens = numberArg(input, "circulatingTokens", numberArg(input, "realToken", 793_100_000));
  const floorPrice = athPrice * floorRatio;
  const floorLiabilitySol = floorPrice * circulatingTokens;
  return {
    unsigned: true,
    model: "rise-style-floor-planning",
    note: "This models a protocol floor for planning. Program code must enforce reserve ownership, borrow caps, and repayment accounting.",
    athPrice,
    floorRatio,
    floorPrice,
    protocolSol,
    circulatingTokens,
    floorLiabilitySol,
    coverageRatio: floorLiabilitySol > 0 ? protocolSol / floorLiabilitySol : null,
    borrow: {
      maxLoanToFloorBps: numberArg(input, "maxLoanToFloorBps", 6500),
      originationFeeBps: numberArg(input, "originationFeeBps", 150),
      liquidation: "none-in-model",
    },
  };
}

function perpPlan(input) {
  const collateralSol = numberArg(input, "collateralSol", 1);
  const leverage = Math.max(1, numberArg(input, "leverage", 2));
  const markPrice = numberArg(input, "markPrice", numberArg(input, "virtualSol", 30) / numberArg(input, "virtualToken", 1_073_000_000));
  const maintenanceMarginBps = numberArg(input, "maintenanceMarginBps", 500);
  const notionalSol = collateralSol * leverage;
  return {
    unsigned: true,
    venue: "p-token-perpetual-planner",
    market: String(input.market ?? `${String(input.symbol ?? "PQC").toUpperCase()}-PERP`),
    side: String(input.perpSide ?? "long"),
    collateralSol,
    leverage,
    notionalSol,
    markPrice,
    baseSize: markPrice > 0 ? notionalSol / markPrice : 0,
    maintenanceMarginBps,
    liquidation: "planner-only; enforce in perp program or external venue",
    riskChecks: [
      "cap leverage by actor type",
      "require oracle freshness",
      "separate launch reserves from perp collateral",
      "reject agent trades without policy approval",
    ],
  };
}

function wdkIntent(input) {
  const actor = String(input.actor ?? "human-or-agent");
  const action = String(input.action ?? "launch-plan");
  return {
    unsigned: true,
    adapter: "wdk-intent",
    actor,
    action,
    chain: "solana",
    network: String(input.network ?? "solana-devnet"),
    walletModule: "@tetherto/wdk-wallet-solana",
    reviewRequired: true,
    instructions: [
      {
        label: "create-or-select-p-token-program",
        programId: String(input.pTokenProgramId ?? DEFAULT_P_TOKEN_PROGRAM_ID),
      },
      {
        label: "create-mint",
        symbol: String(input.symbol ?? "PQC").toUpperCase(),
        decimals: numberArg(input, "decimals", 9),
      },
      {
        label: "initialize-curve",
        virtualSol: numberArg(input, "virtualSol", 30),
        virtualToken: numberArg(input, "virtualToken", 1_073_000_000),
      },
      {
        label: "register-after-verification",
        registry: "data/ptokens.json",
      },
    ],
  };
}

function normalizeList(value) {
  if (Array.isArray(value)) return value.map(String).filter(Boolean);
  return String(value)
    .split(",")
    .map((item) => item.trim())
    .filter(Boolean);
}

function inferExploreKind(query) {
  if (query.length >= 32 && query.length <= 64) return "address";
  if (query.startsWith("http")) return "metadata";
  if (query.includes("-PERP")) return "perp-market";
  return "workspace";
}

function classifyTokenInput(query) {
  return {
    raw: query,
    looksLikeSolanaAddress: /^[1-9A-HJ-NP-Za-km-z]{32,64}$/.test(query),
    looksLikePumpMint: query.endsWith("pump"),
    suggestedActions: ["inspect mint", "build launch plan", "bind to agent", "prepare quote", "draft perp listing"],
  };
}

function recommendedNext(kind) {
  if (kind === "address") return ["inspect mint over RPC", "compare owner program to p-token program id", "derive launchpad PDAs in SDK"];
  if (kind === "metadata") return ["fetch metadata externally", "build agent registration doc", "pin immutable URI"];
  if (kind === "perp-market") return ["verify graduated curve", "generate perps adapter config", "check oracle freshness"];
  return ["create launch plan", "create agent plan", "review devnet program draft"];
}

function draftAddress(...parts) {
  const digest = createHash("sha256").update(parts.join(":")).digest();
  return base58(digest);
}

async function inspectMint(mint, input) {
  if (!mint || mint.length < 32) throw new Error("mint is required");
  const rpcUrl = String(input.rpc ?? DEFAULT_RPC);
  const pTokenProgramId = String(input.pTokenProgramId ?? DEFAULT_P_TOKEN_PROGRAM_ID);
  const account = await rpc(rpcUrl, "getAccountInfo", [mint, { encoding: "base64", commitment: "confirmed" }]);
  if (!account.value) return { exists: false, mint, network: inferNetwork(rpcUrl) };
  const ownerProgram = account.value.owner;
  const data = Buffer.from(account.value.data[0], "base64");
  return {
    exists: true,
    mint,
    network: inferNetwork(rpcUrl),
    ownerProgram,
    tokenProgram: classifyTokenProgram(ownerProgram, pTokenProgramId),
    lamports: account.value.lamports,
    executable: account.value.executable,
    dataLen: data.length,
    mintLayout: data.length >= 82 ? parseMintLayout(data) : null,
  };
}

async function rpc(rpcUrl, method, params) {
  const res = await fetch(rpcUrl, {
    method: "POST",
    headers: { "content-type": "application/json" },
    body: JSON.stringify({ jsonrpc: "2.0", id: "ptoken-launcher-workbench", method, params }),
  });
  if (!res.ok) throw new Error(`${method} HTTP ${res.status}`);
  const json = await res.json();
  if (json.error) throw new Error(`${method}: ${json.error.message ?? JSON.stringify(json.error)}`);
  return json.result;
}

function parseMintLayout(data) {
  const mintAuthorityOption = data.readUInt32LE(0);
  const freezeAuthorityOption = data.readUInt32LE(46);
  return {
    mintAuthority: mintAuthorityOption ? base58(data.subarray(4, 36)) : null,
    supply: data.readBigUInt64LE(36).toString(),
    decimals: data[44],
    isInitialized: data[45] === 1,
    freezeAuthority: freezeAuthorityOption ? base58(data.subarray(50, 82)) : null,
  };
}

function classifyTokenProgram(ownerProgram, pTokenProgramId) {
  if (pTokenProgramId && ownerProgram === pTokenProgramId) return "p-token";
  if (ownerProgram === SPL_TOKEN_PROGRAM_ID) return "spl";
  return "custom";
}

function inferNetwork(rpcUrl) {
  return rpcUrl.includes("mainnet") ? "solana-mainnet" : "solana-devnet";
}

const ALPHABET = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
function base58(bytes) {
  let num = 0n;
  for (const byte of bytes) num = (num << 8n) + BigInt(byte);
  let encoded = "";
  while (num > 0n) {
    const rem = Number(num % 58n);
    num /= 58n;
    encoded = ALPHABET[rem] + encoded;
  }
  for (const byte of bytes) {
    if (byte === 0) encoded = `1${encoded}`;
    else break;
  }
  return encoded || "1";
}
