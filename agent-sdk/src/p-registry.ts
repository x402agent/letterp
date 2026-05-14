export interface AgentService {
  name: string;
  endpoint: string;
  version?: string;
}

export interface AgentRegistrationEntry {
  agentId: string;
  agentRegistry: string;
}

export interface AgentRegistration {
  name: string;
  description: string;
  image: string;
  model: string;
  capabilities: string[];
  endpoint: string;
  services: AgentService[];
  active: boolean;
  registrations: AgentRegistrationEntry[];
  supportedTrust: string[];
}

export function buildAgentRegistrationDoc(
  assetAddress: string,
  opts: AgentRegistration,
): object {
  return {
    "@context": "https://erc8004.org/schema/agent.json",
    "@type": "Agent",
    id: assetAddress,
    name: opts.name,
    description: opts.description,
    image: opts.image,
    model: opts.model,
    capabilities: opts.capabilities,
    endpoint: opts.endpoint,
    services: opts.services.map((s) => ({
      name: s.name,
      endpoint: s.endpoint,
      ...(s.version ? { version: s.version } : {}),
    })),
    active: opts.active,
    registrations: opts.registrations,
    supportedTrust: opts.supportedTrust,
  };
}

export async function fetchAgentRegistration(
  uri: string,
): Promise<AgentRegistration | null> {
  try {
    const res = await fetch(uri);
    if (!res.ok) return null;
    const doc = await res.json();
    if (!validateAgentRegistration(doc)) return null;
    return doc;
  } catch {
    return null;
  }
}

export function validateAgentRegistration(
  doc: unknown,
): doc is AgentRegistration {
  if (typeof doc !== "object" || doc === null) return false;
  const d = doc as Record<string, unknown>;
  if (typeof d["name"] !== "string") return false;
  if (typeof d["description"] !== "string") return false;
  if (typeof d["image"] !== "string") return false;
  if (typeof d["model"] !== "string") return false;
  if (!Array.isArray(d["capabilities"])) return false;
  if (typeof d["endpoint"] !== "string") return false;
  if (!Array.isArray(d["services"])) return false;
  if (typeof d["active"] !== "boolean") return false;
  if (!Array.isArray(d["registrations"])) return false;
  if (!Array.isArray(d["supportedTrust"])) return false;
  return true;
}
