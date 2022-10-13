import { StackArray } from './stack-array'

describe("stack array", () => {
  const stack = new StackArray()

  test("isEmpty", () => {
    expect(stack.isEmpty()).toBe(true)
  })

  test("push & peek", () => {
    stack.push(5)
    stack.push(10)
    expect(stack.peek()).toBe(10)
    stack.push(123)
    expect(stack.peek()).toBe(123)
    expect(stack.size()).toBe(3)
    expect(stack.isEmpty()).toBe(false)
  })
})
