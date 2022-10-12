import { StackObject } from './stack-object'

describe("test stack.ts", () => {
  const stack = new StackObject()


  test("test the push method", () => {
    stack.push(1)
    stack.push(2)
    expect(stack.items[0]).toBe(1)
  })

  test("test the size method", () => {
    stack.push(5)
    expect(stack.size()).toBe(3)
  })

  test("test the isEmpty method", () => {
    expect(stack.isEmpty()).toBe(false)
  })

  test("test the pop method", () => {
    const result = stack.pop();
    expect(result).toBe(5)
    expect(stack.size()).toBe(2)
  })

  test("test the peek method", () => {
    const lastItem = stack.peek()
    expect(lastItem).toBe(2)
  })

  test("test the toString method", () => {
    stack.push('h')
    stack.push('e')
    expect(stack.toString()).toBe('1,2,h,e')
  })

  test("test the clear method", () => {
    stack.clear()
    expect(stack.items).toMatchObject({})
    expect(stack.size()).toBe(0)
  })
})
