export interface OntologyStatement {
  object: string;
  predicate: string;
}

export type OntologyEntityKind = "type" | "predicate";

export interface OntologyEntity {
  attributes: string[];
  children: string[];
  description: string | null;
  displayName: string;
  id: string;
  kind: OntologyEntityKind;
  parents: string[];
  rawBlock: string;
  statements: OntologyStatement[];
  subject: string;
  targetTypes: string[];
}
