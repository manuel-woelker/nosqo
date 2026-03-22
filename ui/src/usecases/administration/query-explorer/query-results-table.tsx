import type { QueryResponse } from "../../../infrastructure/api/api-client";
import { NosqoEmptyState } from "../../../common/components/nosqo-empty-state";

export function QueryResultsTable({ result }: { result: QueryResponse }) {
  if (result.rows.length === 0) {
    return (
      <NosqoEmptyState
        body="The query executed successfully, but nothing matched this pattern."
        title="No matching rows"
      />
    );
  }

  return (
    <div className="table-shell">
      <table>
        <caption className="sr-only">NQL query results</caption>
        <thead>
          <tr>
            {result.columns.map((column) => (
              <th key={column} scope="col">
                {column}
              </th>
            ))}
          </tr>
        </thead>
        <tbody>
          {result.rows.map((row, rowIndex) => (
            <tr key={`${rowIndex}-${row.join("|")}`}>
              {row.map((value, valueIndex) => (
                <td key={`${rowIndex}-${valueIndex}`}>{value}</td>
              ))}
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
