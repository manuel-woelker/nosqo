export interface QueryResponse {
  columns: string[];
  rows: string[][];
}

export interface EntitySearchRequest {
  filters: Record<string, string>;
  type: string;
}

export interface EntitySearchResult {
  id: string;
  label: string;
  nosqoId: string;
  typeIds: string[];
}

export interface EntitySearchResponse {
  results: EntitySearchResult[];
}

export interface EntityDetailAttribute {
  label: string;
  predicateId: string;
  values: string[];
}

export interface EntityDetailResponse {
  attributes: EntityDetailAttribute[];
  id: string;
  label: string;
  nosqoId: string;
  typeIds: string[];
}

export interface StatementFilters {
  subject?: string;
  predicate?: string;
  object?: string;
}

export type NosqoStatementJsonValue = string | [string];

export interface NosqoStatementJsonDocument {
  format: string;
  values: NosqoStatementJsonValue[];
  statements: number[][];
}

export class ApiError extends Error {
  readonly status: number;

  constructor(message: string, status: number) {
    super(message);
    this.name = "ApiError";
    this.status = status;
  }
}

export async function fetchNqlQuery(queryText: string): Promise<QueryResponse> {
  const response = await fetch(buildApiUrl("/api/v1/query"), {
    method: "POST",
    headers: {
      "Content-Type": "text/plain",
      Accept: "application/json",
    },
    body: queryText,
  });

  return readJsonResponse<QueryResponse>(response, "query");
}

export async function fetchEntitySearch(
  request: EntitySearchRequest,
): Promise<EntitySearchResponse> {
  const response = await fetch(buildApiUrl("/api/v1/entities/search"), {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      Accept: "application/json",
    },
    body: JSON.stringify(request),
  });

  return readJsonResponse<EntitySearchResponse>(response, "entity browser");
}

export async function fetchEntityDetail(entityId: string): Promise<EntityDetailResponse> {
  const response = await fetch(buildApiUrl(`/api/v1/entities/${encodeURIComponent(entityId)}`), {
    headers: {
      Accept: "application/json",
    },
  });

  return readJsonResponse<EntityDetailResponse>(response, "entity browser");
}

export async function fetchStatements(filters: StatementFilters): Promise<string> {
  const url = new URL(buildApiUrl("/api/v1/statements"), window.location.origin);

  for (const [key, value] of Object.entries(filters)) {
    if (value && value.trim().length > 0) {
      url.searchParams.set(key, value.trim());
    }
  }

  const response = await fetch(url, {
    headers: {
      Accept: "text/plain",
    },
  });

  if (!response.ok) {
    throw new ApiError(await readErrorMessage(response, "statement browser"), response.status);
  }

  return response.text();
}

export async function fetchOntologyStatementJson(): Promise<NosqoStatementJsonDocument> {
  const response = await fetch(buildApiUrl("/api/v1/ontology"), {
    headers: {
      Accept: "application/json",
    },
  });

  return readJsonResponse<NosqoStatementJsonDocument>(response, "ontology viewer");
}

function buildApiUrl(path: string): string {
  const configuredBaseUrl = import.meta.env.VITE_API_BASE_URL;

  if (configuredBaseUrl && configuredBaseUrl.length > 0) {
    return new URL(path, configuredBaseUrl).toString();
  }

  return path;
}

async function readJsonResponse<T>(response: Response, featureName: string): Promise<T> {
  if (!response.ok) {
    throw new ApiError(await readErrorMessage(response, featureName), response.status);
  }

  return (await response.json()) as T;
}

async function readErrorMessage(response: Response, featureName: string): Promise<string> {
  const contentType = response.headers.get("content-type") ?? "";

  if (contentType.includes("application/json")) {
    const payload = (await response.json()) as { error?: string };

    if (payload.error && payload.error.length > 0) {
      return payload.error;
    }
  }

  const fallbackMessage = await response.text();

  if (fallbackMessage.length > 0) {
    return fallbackMessage;
  }

  return `The ${featureName} request failed with status ${response.status}.`;
}
