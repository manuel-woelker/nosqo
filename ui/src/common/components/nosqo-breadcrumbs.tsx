import { Breadcrumbs, Text } from "@mantine/core";
import { Link } from "@tanstack/react-router";
import type { BreadcrumbItem } from "../../infrastructure/routing/navigation-model";

export function NosqoBreadcrumbs({ items }: { items: BreadcrumbItem[] }) {
  return (
    <nav aria-label="Breadcrumbs">
      <Breadcrumbs className="breadcrumb-list" separator="/">
        {items.map((item, index) =>
          item.href ? (
            <Link className="breadcrumb-link" key={`${item.label}-${index}`} to={item.href}>
              {item.label}
            </Link>
          ) : (
            <Text className="breadcrumb-current" key={`${item.label}-${index}`} size="sm">
              {item.label}
            </Text>
          ),
        )}
      </Breadcrumbs>
    </nav>
  );
}
