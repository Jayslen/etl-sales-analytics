import { SQL } from 'bun'

const db = DB()

const server = Bun.serve({
  routes: {
    "/customers": async (req, res) => {
      const url = new URL(req.url)
      const { offset, limit } = getParams(req)

      try {
        const result = await db`SELECT * FROM customers LIMIT ${limit} OFFSET ${offset}`

        return Response.json(result)
      } catch (error) {
        return new Response("Unexpected Server Error", { status: 500 })
      }
    },
    "/products": async (req, res) => {
      const url = new URL(req.url)
      const { offset, limit } = getParams(req)

      try {
        const result = await db`SELECT * FROM products LIMIT ${limit} OFFSET ${offset}`
        return Response.json(result)
      } catch (error) {
        return new Response("Unexpected Server Error", { status: 500 })
      }
    }
  },
  port: 3000,
  fetch(req) {
    return new Response("Not Found", { status: 404 })
  }
})

console.log(`Server running on http://localhost:${server.port}`)

function DB() {
  const connectionKey = Bun.env.BUN_CONNECTION
  const db = new SQL(connectionKey as string)

  return db
}

function getParams(req: Request) {
  const url = new URL(req.url)

  const offset = Number(url.searchParams.get("offset") ?? 0)
  const limit = Number(url.searchParams.get("limit") ?? 5)

  return { offset, limit }
}
