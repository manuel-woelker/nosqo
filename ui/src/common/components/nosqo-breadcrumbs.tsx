import { Link } from "@tanstack/react-router";
import type { BreadcrumbItem } from "../../infrastructure/routing/navigation-model";

export function NosqoBreadcrumbs({ items }: { items: BreadcrumbItem[] }) {
  return (
    <nav aria-label="Breadcrumbs">
      <ol className="breadcrumb-list">
        {items.map((item, index) =>
          item.href ? (
            <li className="breadcrumb-item" key={`${item.label}-${index}`}>
              <Link className="breadcrumb-link" to={item.href}>
                {item.label}
              </Link>
            </li>
          ) : (
            <li className="breadcrumb-item" key={`${item.label}-${index}`}>
              <span className="breadcrumb-current">{item.label}</span>
            </li>
          ),
        )}
      </ol>
    </nav>
  );
}
