export { LspBridgeContainer } from "./container";

export default {
  async fetch(request: Request, env: Env): Promise<Response> {
    if (request.headers.get("Upgrade") === "websocket") {
      const id = env.LSP_CONTAINER.newUniqueId();
      return env.LSP_CONTAINER.get(id).fetch(request);
    }

    const url = new URL(request.url);
    if (url.pathname === "/") {
      url.pathname = "/index.html";
      request = new Request(url, request);
    }
    return env.ASSETS.fetch(request);
  },
} satisfies ExportedHandler<Env>;
