import { routePaths } from "./route-paths";

export interface BreadcrumbItem {
  href?: string;
  label: string;
}

export interface NavigationItem {
  href: string;
  label: string;
  eyebrow: string;
}

export interface NavigationGroup {
  items: NavigationItem[];
  label: string;
}

export const navigationGroups: NavigationGroup[] = [
  {
    label: "Administration",
    items: [
      {
        label: "Ontology",
        eyebrow: "Model",
        href: routePaths.ontology,
      },
      {
        label: "Query Explorer",
        eyebrow: "NQL",
        href: routePaths.queryExplorer,
      },
      {
        label: "Statement Browser",
        eyebrow: "Store",
        href: routePaths.statementBrowser,
      },
    ],
  },
];

export function getBreadcrumbItems(pathname: string): BreadcrumbItem[] {
  switch (pathname) {
    case routePaths.ontology:
      return [
        { href: routePaths.home, label: "Home" },
        { label: "Administration" },
        { label: "Ontology" },
      ];
    case routePaths.queryExplorer:
      return [
        { href: routePaths.home, label: "Home" },
        { label: "Administration" },
        { label: "Query Explorer" },
      ];
    case routePaths.statementBrowser:
      return [
        { href: routePaths.home, label: "Home" },
        { label: "Administration" },
        { label: "Statement Browser" },
      ];
    case routePaths.home:
    default:
      return [{ label: "Home" }];
  }
}

export function isNavigationItemActive(pathname: string, href: string): boolean {
  return pathname === href;
}
