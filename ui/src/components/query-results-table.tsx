import type { QueryResponse } from "../lib/api-client";

export function QueryResultsTable({ result }: { result: QueryResponse }) {
  if (result.rows.length === 0) {
    return (
      <div className="empty-state">
        <h3>No matching rows</h3>
        <p>The query executed successfully, but nothing matched this pattern.</p>
      </div>
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
