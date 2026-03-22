import { act, render } from "@testing-library/react";
import { RouterProvider, createMemoryHistory, type AnyRouter } from "@tanstack/react-router";
import { createAppRouter } from "../router";

export async function renderApp(initialEntries: string[]) {
  const router = createAppRouter(
    createMemoryHistory({
      initialEntries,
    }),
  );
  let rendered = null as ReturnType<typeof render> | null;

  await act(async () => {
    await router.load();
    rendered = render(<RouterProvider router={router} />);
  });

  return {
    router,
    ...(rendered as ReturnType<typeof render>),
  };
}

export async function navigateTo(router: AnyRouter, to: string) {
  await act(async () => {
    await router.navigate({ to });
  });
}
