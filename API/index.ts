import { SQL } from 'bun'

const db = DB()

const server = Bun.serve({
  routes: {
    "/customers": async (req, res) => {
      const url = new URL(req.url)
      const { offset, limit } = getParams(req)

      try {
        const result = await db`
        SELECT
        c.customer_id, c.first_name, c.last_name, c.email,
          c.phone, ct.city_name as city, cr.country_name as country
        FROM customers AS c
        INNER JOIN cities as ct
        ON c.city_id = ct.city_id
        LEFT JOIN countries as cr
        ON cr.country_id = ct.country_id
        LIMIT ${limit} OFFSET ${offset};
`
        return Response.json(result)
      } catch (error) {
        console.log(error)
        return new Response("Unexpected Server Error", { status: 500 })
      }
    },
    "/products": async (req, res) => {
      const url = new URL(req.url)
      const { offset, limit } = getParams(req)

      try {
        const result = await db`SELECT
          product_id, product_name, c.category_name as category, price, stock
        FROM products AS p
        INNER JOIN categories As c
        ON p.category_id = c.category_id
        LIMIT ${limit} OFFSET ${offset}`
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
