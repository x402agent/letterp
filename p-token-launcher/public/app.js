const state = {
  launch: {},
  agent: {},
  program: {},
};

const outputIds = [
  "explore-output",
  "plan-output",
  "agent-output",
  "program-output",
  "quote-output",
  "perp-output",
  "inspect-output",
  "registry-output",
];

document.querySelectorAll(".tabs button").forEach((button) => {
  button.addEventListener("click", () => activateOutput(button.dataset.output));
});

document.getElementById("workspace-button").addEventListener("click", async () => {
  render("explore-output", await getJson("/api/workspace"));
});

document.getElementById("explore-form").addEventListener("submit", async (event) => {
  event.preventDefault();
  render("explore-output", await postJson("/api/explore", formData(event.currentTarget)));
});

document.getElementById("launch-form").addEventListener("submit", async (event) => {
  event.preventDefault();
  state.launch = await postJson("/api/launch-plan", formData(event.currentTarget));
  render("plan-output", state.launch);
});

document.getElementById("agent-form").addEventListener("submit", async (event) => {
  event.preventDefault();
  const body = { ...state.launch?.metadata, ...formData(event.currentTarget) };
  state.agent = await postJson("/api/agent-plan", body);
  render("agent-output", state.agent);
});

document.getElementById("program-form").addEventListener("submit", async (event) => {
  event.preventDefault();
  state.program = await postJson("/api/program-draft", formData(event.currentTarget));
  render("program-output", state.program);
});

document.getElementById("quote-form").addEventListener("submit", async (event) => {
  event.preventDefault();
  const body = { ...state.launch?.bondingCurve, ...formData(event.currentTarget) };
  render("quote-output", await postJson("/api/quote", body));
});

document.getElementById("perp-form").addEventListener("submit", async (event) => {
  event.preventDefault();
  const metadata = state.launch?.metadata ?? {};
  const curve = state.launch?.bondingCurve ?? {};
  render("perp-output", await postJson("/api/perp-plan", { ...metadata, ...curve, ...formData(event.currentTarget) }));
});

document.getElementById("inspect-form").addEventListener("submit", async (event) => {
  event.preventDefault();
  render("inspect-output", await postJson("/api/inspect", formData(event.currentTarget)));
});

await boot();

async function boot() {
  const [health, registry, workspace] = await Promise.all([
    getJson("/api/health"),
    getJson("/api/registry"),
    getJson("/api/workspace"),
  ]);
  document.getElementById("status").textContent =
    `${health.network} / ${health.unsigned ? "unsigned" : "signing"} / ${workspace.packages.length} modules`;
  render("registry-output", registry);
  render("explore-output", workspace);
  document.getElementById("launch-form").requestSubmit();
  document.getElementById("agent-form").requestSubmit();
  document.getElementById("program-form").requestSubmit();
}

function activateOutput(id) {
  outputIds.forEach((outputId) => {
    document.getElementById(outputId).classList.toggle("active", outputId === id);
  });
  document.querySelectorAll(".tabs button").forEach((button) => {
    button.classList.toggle("active", button.dataset.output === id);
  });
}

function render(id, payload) {
  document.getElementById(id).textContent = JSON.stringify(payload, null, 2);
  activateOutput(id);
}

function formData(form) {
  const data = new FormData(form);
  const out = {};
  for (const [key, value] of data.entries()) {
    if (value === "") continue;
    const field = form.elements[key];
    const type = field instanceof RadioNodeList ? "radio" : field?.type;
    out[key] = type === "number" ? Number(value) : value;
  }
  return out;
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
