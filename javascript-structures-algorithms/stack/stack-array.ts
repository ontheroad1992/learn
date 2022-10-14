export class StackArray<E extends any> {
  items: E [];

  constructor() {
    this.items = []
  }

  push(element: E) {
    this.items.push(element)
  }

  pop() {
    if (this.isEmpty()) {
      return ''
    }
    return this.items.pop()
  }

  // 查看栈顶元素
  peek() {
    return this.items[this.items.length - 1]
  }

  isEmpty() {
    return this.items.length === 0;
  }

  clear() {
    this.items = []
  }

  size() {
    return this.items.length
  }
}
