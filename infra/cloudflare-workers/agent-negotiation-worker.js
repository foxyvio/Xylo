export default {
  async fetch(request, env, ctx) {
    const url = new URL(request.url);
    
    // Simple edge routing for agent discovery
    if (url.pathname === "/negotiate") {
      return new Response(JSON.stringify({
        status: "negotiating",
        message: "Agent negotiation initiated at the edge.",
        timestamp: new Date().toISOString()
      }), {
        headers: { "content-type": "application/json" }
      });
    }

    return new Response("Xylo Edge Negotiation Layer", { status: 200 });
  },
};
