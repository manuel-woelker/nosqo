import { useEffect, type ReactNode } from "react";
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
    <div className="nosqo-shell">
      <header className="shell-header">
        <div className="shell-header__row">
          <button
            aria-label="Toggle navigation"
            className="shell-burger"
            onClick={toggleNavigation}
            type="button"
          >
            <span
              className={
                isNavigationOpen
                  ? "shell-burger__lines shell-burger__lines--open"
                  : "shell-burger__lines"
              }
            />
          </button>
          <div className="brand-copy">
            <Link className="brand-link" to={routePaths.home}>
              nosqo
            </Link>
          </div>
          <NosqoBreadcrumbs items={breadcrumbs} />
        </div>
      </header>

      <div className="shell-body">
        <nav
          aria-label="Primary navigation"
          className={isNavigationOpen ? "shell-navbar shell-navbar--open" : "shell-navbar"}
        >
          <div className="shell-navbar__scroll">
            {navigationGroups.map((group) => (
              <section aria-label={group.label} key={group.label}>
                <p className="nav-group-label">{group.label}</p>
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
                        <span className="nav-link__label">{item.label}</span>
                      </Link>
                    );
                  })}
                </div>
              </section>
            ))}
          </div>
        </nav>

        <main className="shell-main">
          {isNavigationOpen ? (
            <button
              aria-label="Close navigation"
              className="shell-backdrop"
              onClick={closeNavigation}
              type="button"
            />
          ) : null}
          <div className="shell-main__viewport">
            <div className="shell-main__content">{children}</div>
          </div>
        </main>
      </div>
    </div>
  );
}
