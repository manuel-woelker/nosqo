import type { ErrorComponentProps } from "@tanstack/react-router";
import { Link } from "@tanstack/react-router";
import { NosqoPanel } from "../../common/components/nosqo-panel";
import { routePaths } from "../../infrastructure/routing/route-paths";

export function RouteErrorPage({ error }: ErrorComponentProps) {
  return (
    <NosqoPanel className="panel-stack">
      <p className="kicker">Route error</p>
      <h1 className="page-title">The UI tripped over itself.</h1>
      <p className="body-copy">{error.message}</p>
      <div>
        <Link className="button-link" to={routePaths.home}>
          Back to safety
        </Link>
      </div>
    </NosqoPanel>
  );
}
