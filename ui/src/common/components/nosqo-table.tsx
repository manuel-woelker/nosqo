import type { ReactNode } from "react";

export interface NosqoTableColumn {
  id: string;
  label: string;
}

export interface NosqoTableRow {
  cells: ReactNode[];
  key: string;
}

/**
 * Renders a compact semantic table with consistent nosqo styling.
 */
export function NosqoTable({
  columns,
  rows,
  className,
}: {
  columns: NosqoTableColumn[];
  rows: NosqoTableRow[];
  className?: string;
}) {
  const nextClassName = className ? `nosqo-table ${className}` : "nosqo-table";

  return (
    <table className={nextClassName}>
      <thead>
        <tr>
          {columns.map((column) => (
            <th key={column.id} scope="col">
              {column.label}
            </th>
          ))}
        </tr>
      </thead>
      <tbody>
        {rows.map((row) => (
          <tr key={row.key}>
            {row.cells.map((cell, index) => (
              <td key={`${row.key}-${columns[index]?.id ?? index}`}>{cell}</td>
            ))}
          </tr>
        ))}
      </tbody>
    </table>
  );
}
