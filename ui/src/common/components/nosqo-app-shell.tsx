import { useEffect, type ReactNode } from "react";
import { AppShell, Burger, Group, ScrollArea, Stack, Text } from "@mantine/core";
import { Link, useRouterState } from "@tanstack/react-router";
import {
  getBreadcrumbItems,
  isNavigationItemActive,
  navigationGroups,
} from "../../infrastructure/routing/navigation-model";
import { routePaths } from "../../infrastructure/routing/route-paths";
import { useUiShellStore } from "../../infrastructure/state/ui-shell-store";
import { NosqoBreadcrumbs } from "./nosqo-breadcrumbs";

export function NosqoAppShell({ children }: { children: ReactNode }) {
  const pathname = useRouterState({
    select: (state) => state.location.pathname,
  });
  const breadcrumbs = getBreadcrumbItems(pathname);
  const isNavigationOpen = useUiShellStore((state) => state.isNavigationOpen);
  const closeNavigation = useUiShellStore((state) => state.closeNavigation);
  const toggleNavigation = useUiShellStore((state) => state.toggleNavigation);

  useEffect(() => {
    closeNavigation();
  }, [closeNavigation, pathname]);

  return (
    <AppShell
      className="nosqo-shell"
      header={{ height: 88 }}
      navbar={{
        width: 320,
        breakpoint: "sm",
        collapsed: {
          desktop: false,
          mobile: !isNavigationOpen,
        },
      }}
      padding="lg"
    >
      <AppShell.Header className="shell-header">
        <Group className="shell-header__row" gap="md" h="100%" justify="space-between" px="lg">
          <Group gap="md" wrap="nowrap">
            <Burger
              aria-label="Toggle navigation"
              hiddenFrom="sm"
              onClick={toggleNavigation}
              opened={isNavigationOpen}
              size="sm"
            />
            <div className="brand-copy">
              <Link className="brand-link" to={routePaths.home}>
                nosqo
              </Link>
              <Text className="brand-subtitle" size="sm">
                Knowledge tools
              </Text>
            </div>
          </Group>
          <NosqoBreadcrumbs items={breadcrumbs} />
        </Group>
      </AppShell.Header>

      <AppShell.Navbar aria-label="Primary navigation" className="shell-navbar" p="md">
        <ScrollArea className="shell-navbar__scroll" type="never">
          <Stack gap="xl">
            {navigationGroups.map((group) => (
              <section aria-label={group.label} key={group.label}>
                <Text className="nav-group-label" size="xs">
                  {group.label}
                </Text>
                <div className="nav-list">
                  {group.items.map((item) => {
                    const isActive = isNavigationItemActive(pathname, item.href);
                    const linkClassName = isActive ? "nav-link nav-link--active" : "nav-link";

                    return (
                      <Link
                        className={linkClassName}
                        data-active={isActive ? "true" : "false"}
                        key={item.href}
                        onClick={closeNavigation}
                        to={item.href}
                      >
                        <span className="nav-link__eyebrow">{item.eyebrow}</span>
                        <span className="nav-link__label">{item.label}</span>
                      </Link>
                    );
                  })}
                </div>
              </section>
            ))}
          </Stack>
        </ScrollArea>
      </AppShell.Navbar>

      <AppShell.Main className="shell-main">
        <div className="shell-main__content">{children}</div>
      </AppShell.Main>
    </AppShell>
  );
}
