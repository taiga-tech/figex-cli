// MCP transport — stub.
//
// The transport interface is designed to accept an MCP adapter from the start.
// The first implementation stage prioritises CDP; MCP will be added in a later
// stage using the same `RuntimeClient` trait.
//
// Figma publishes two MCP endpoints (as of 2026-03-22):
//   desktop server : http://127.0.0.1:3845/mcp
//   remote server  : https://mcp.figma.com/mcp
