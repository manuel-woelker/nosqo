import type {
  NosqoStatementJsonDocument,
  NosqoStatementJsonValue,
} from "../../../infrastructure/api/api-client";
import type { OntologyEntity, OntologyEntityKind, OntologyStatement } from "./ontology-types";

export function transformOntologyStatementJson(
  document: NosqoStatementJsonDocument,
): OntologyEntity[] {
  if (document.format !== "nosqo-statement-json-v1") {
    throw new Error(`Unsupported ontology statement format: ${document.format}`);
  }

  const groupedStatements = new Map<string, OntologyStatement[]>();

  for (const row of document.statements) {
    if (row.length < 3) {
      throw new Error(
        "Ontology statement rows must contain at least subject, predicate, and object indexes.",
      );
    }

    const subject = decodeNosqoValue(document.values, row[0]).rawNosqo;
    const predicate = normalizePredicateName(decodeNosqoValue(document.values, row[1]).rawNosqo);

    for (const objectIndex of row.slice(2)) {
      const object = decodeNosqoValue(document.values, objectIndex);
      const statement: OntologyStatement = {
        predicate,
        object: object.displayValue,
        rawObject: object.rawNosqo,
      };

      const statementsForSubject = groupedStatements.get(subject) ?? [];
      statementsForSubject.push(statement);
      groupedStatements.set(subject, statementsForSubject);
    }
  }

  const baseEntities = Array.from(groupedStatements.entries())
    .map(([subject, statements]) => createOntologyEntity(subject, statements))
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

function createOntologyEntity(subject: string, statements: OntologyStatement[]): OntologyEntity {
  const kind = inferKind(subject);
  const displayName =
    readStatementValue(statements, "label") ??
    readStatementValue(statements, "name") ??
    subject.replace(/^[#~@]/, "");
  const description = readStatementValue(statements, "description");
  const parents = statements
    .filter((statement) => statement.predicate === "isA")
    .map((statement) => statement.rawObject);
  const attributes = statements
    .filter((statement) => statement.predicate === "attribute")
    .map((statement) => statement.rawObject);
  const targetTypes = statements
    .filter(
      (statement) => statement.predicate === "targetType" || statement.predicate === "valueType",
    )
    .map((statement) => statement.rawObject);

  return {
    id: subject.replace(/^[#~@]/, ""),
    subject,
    kind,
    displayName,
    description,
    parents,
    children: [],
    attributes,
    targetTypes,
    statements,
    rawBlock: renderRawBlock(subject, statements),
  };
}

function decodeNosqoValue(values: NosqoStatementJsonValue[], index: number): DecodedNosqoValue {
  const value = values[index];

  if (value === undefined) {
    throw new Error(`Ontology value index ${index} is out of bounds.`);
  }

  if (typeof value === "string") {
    return {
      displayValue: value,
      rawNosqo: value,
    };
  }

  if (Array.isArray(value) && value.length === 1) {
    return {
      displayValue: value[0],
      rawNosqo: `"${escapeDoubleQuoted(value[0])}"`,
    };
  }

  throw new Error(`Invalid ontology value entry at index ${index}.`);
}

function normalizePredicateName(rawPredicate: string): string {
  return rawPredicate.startsWith("~") ? rawPredicate.slice(1) : rawPredicate;
}

function inferKind(subject: string): OntologyEntityKind {
  if (subject.startsWith("~")) {
    return "predicate";
  }

  return "type";
}

function readStatementValue(statements: OntologyStatement[], predicate: string): string | null {
  const statement = statements.find((entry) => entry.predicate === predicate);

  return statement?.object ?? null;
}

function renderRawBlock(subject: string, statements: OntologyStatement[]): string {
  const lines = statements.map((statement) => `  ${statement.predicate} ${statement.rawObject}`);

  return `${subject} {\n${lines.join("\n")}\n}`;
}

function kindSortOrder(kind: OntologyEntityKind): number {
  if (kind === "type") {
    return 0;
  }

  return 1;
}

function escapeDoubleQuoted(value: string): string {
  return value
    .replace(/\\/g, "\\\\")
    .replace(/\n/g, "\\n")
    .replace(/\r/g, "\\r")
    .replace(/\t/g, "\\t")
    .replace(/"/g, '\\"');
}

interface DecodedNosqoValue {
  displayValue: string;
  rawNosqo: string;
}
