import { Link } from "@tanstack/react-router";
import { NosqoPanel } from "../../common/components/nosqo-panel";
import { routePaths } from "../../infrastructure/routing/route-paths";

export function NotFoundPage() {
  return (
    <NosqoPanel className="panel-stack">
      <p className="kicker">404</p>
      <h1 className="page-title">This route wandered off the graph.</h1>
      <p className="body-copy">
        The UI only ships a few administration routes right now. The rest are still vapor.
      </p>
      <div>
        <Link className="button-link" to={routePaths.home}>
          Return home
        </Link>
      </div>
    </NosqoPanel>
  );
}
