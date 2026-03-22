import type { OntologyEntity, OntologyEntityKind, OntologyStatement } from "./ontology-types";

export function parseOntologyNosqo(source: string): OntologyEntity[] {
  const trimmedSource = source.trim();

  if (trimmedSource.length === 0) {
    return [];
  }

  const baseEntities = trimmedSource
    .split(/\n\s*\n/)
    .map((block) => parseOntologyBlock(block))
    .sort((left, right) => {
      if (left.kind !== right.kind) {
        return kindSortOrder(left.kind) - kindSortOrder(right.kind);
      }

      return left.displayName.localeCompare(right.displayName);
    });

  return baseEntities.map((entity) => ({
    ...entity,
    children: baseEntities
      .filter((candidate) => candidate.parents.includes(entity.subject))
      .map((candidate) => candidate.subject)
      .sort((left, right) => left.localeCompare(right)),
  }));
}

function parseOntologyBlock(block: string): OntologyEntity {
  const lines = block
    .split("\n")
    .map((line) => line.trim())
    .filter((line) => line.length > 0);

  const headerLine = lines.at(0);

  if (!headerLine || !headerLine.endsWith("{")) {
    throw new Error(`Invalid ontology block header: ${block}`);
  }

  const subject = headerLine.slice(0, -1).trim();
  const statements = lines
    .slice(1)
    .filter((line) => line !== "}")
    .map((line) => parseStatementLine(line));

  const kind = inferKind(subject);
  const displayName =
    readLiteralValue(statements, "label") ??
    readLiteralValue(statements, "name") ??
    subject.replace(/^[#~]/, "");
  const description = readLiteralValue(statements, "description");
  const parents = statements
    .filter((statement) => statement.predicate === "isA")
    .map((statement) => statement.object);
  const attributes = statements
    .filter((statement) => statement.predicate === "attribute")
    .map((statement) => statement.object);
  const targetTypes = statements
    .filter(
      (statement) => statement.predicate === "targetType" || statement.predicate === "valueType",
    )
    .map((statement) => statement.object);

  return {
    id: subject.replace(/^[#~]/, ""),
    subject,
    kind,
    displayName,
    description,
    parents,
    children: [],
    attributes,
    targetTypes,
    statements,
    rawBlock: block.trim(),
  };
}

function parseStatementLine(line: string): OntologyStatement {
  const separatorIndex = line.indexOf(" ");

  if (separatorIndex === -1) {
    throw new Error(`Invalid ontology statement line: ${line}`);
  }

  return {
    predicate: line.slice(0, separatorIndex).trim(),
    object: line.slice(separatorIndex + 1).trim(),
  };
}

function inferKind(subject: string): OntologyEntityKind {
  if (subject.startsWith("~")) {
    return "predicate";
  }

  return "type";
}

function kindSortOrder(kind: OntologyEntityKind): number {
  if (kind === "type") {
    return 0;
  }

  return 1;
}

function readLiteralValue(statements: OntologyStatement[], predicate: string): string | null {
  const match = statements.find((statement) => statement.predicate === predicate);

  if (!match) {
    return null;
  }

  return normalizeOntologyValue(match.object);
}

export function normalizeOntologyValue(value: string): string {
  if (
    (value.startsWith('"') && value.endsWith('"')) ||
    (value.startsWith("'") && value.endsWith("'"))
  ) {
    return value.slice(1, -1).replace(/\\n/g, "\n").replace(/\\"/g, '"').replace(/\\'/g, "'");
  }

  return value;
}
