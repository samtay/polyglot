# Basic linked list taken straight from
# https://medium.com/@zylberberg.jonathan/linked-lists-f656bd22f2fa

class Node
  attr_accessor :value, :next

  def initialize(value, next_node)
      @value = value
      @next = next_node
  end
end

class LinkedList
  attr_reader :head

  def initialize(value)
    @head = Node.new(value, nil)
  end

  def from_list(list)
    prev_node = nil
    list.reverse.each { |value|
      prev_node = Node.new(value, prev_node)
    }
    @head = prev_node
  end

  def nth_node(n)
    current_node = @head
    i = 0
    while i < n
      current_node = current_node.next
      i += 1
    end
    return current_node
  end

  # assumes no cycles
  def add_node(node)
    current_node = @head
    while current_node.next != nil
      current_node = current_node.next
    end
    current_node.next = node
    return self
  end

  # assumes no cycles
  def add_value(value)
    self.add_node(Node.new(value, nil))
    return self
  end

  def as_list
    elements = []
    current_node = @head
    while current_node.next != nil
      elements << current_node
      current_node = current_node.next
    end
    elements << current
  end
end
