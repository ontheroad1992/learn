import Stack from './stack-array'

describe("stack array", () => {
  const stack = new Stack()

  test("isEmpty", () => {
    expect(stack.isEmpty()).toBe(true)
  })

  test("push & peek", () => {
    stack.push(5)
    stack.push(10)
    expect(stack.peek()).toBe(10)
  })
})
