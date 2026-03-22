import { transformOntologyStatementJson } from "./transform-ontology-statement-json";

describe("transform ontology statement json", () => {
  it("projects indexed transport into ontology entities", () => {
    const entities = transformOntologyStatementJson({
      format: "nosqo-statement-json-v1",
      values: [
        "#Person",
        "~attribute",
        "~description",
        "~label",
        "~isA",
        "#Type",
        ["Person"],
        ["A human individual."],
        "~name",
        "#Predicate",
        "~targetType",
        "#String",
      ],
      statements: [
        [0, 1, 2, 8],
        [0, 4, 5],
        [0, 3, 6],
        [0, 2, 7],
        [8, 4, 9],
        [8, 10, 11],
      ],
    });

    expect(entities).toHaveLength(2);
    expect(entities[0]).toMatchObject({
      subject: "#Person",
      displayName: "Person",
      kind: "type",
      parents: ["#Type"],
      attributes: ["~description", "~name"],
    });
    expect(entities[1]).toMatchObject({
      subject: "~name",
      kind: "predicate",
      targetTypes: ["#String"],
    });
  });
});
