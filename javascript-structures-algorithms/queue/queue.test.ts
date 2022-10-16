import { Queue } from './queue'

describe("test the Queue", () => {
  const queue = new Queue()


  it("test the queue private value", () => {
    expect(Object.getOwnPropertyNames(queue)).toMatchObject({})
  })

  it('test the peek method', () => {
    expect(queue.peek()).toBe(undefined)
  })

  it('test the enqueue method', () => {
    queue.enqueue('hello')
    expect(queue.peek()).toBe('hello')
  })

  it('test the dequeue method', () => {
    queue.dequeue()
    expect(queue.peek()).toBe(undefined)
  })

  it('test the size method', () => {
    queue.enqueue('hello')
    queue.enqueue('world')
    expect(queue.size()).toBe(2)
  })

  it('test the dequeue method', () => {
    queue.dequeue()
    expect(queue.size()).toBe(1)
    expect(queue.peek()).toBe("world")
  })

  it('test the toString method', () => {
    queue.enqueue('china')
    expect(queue.toString()).toBe('world,china')
  })
})
