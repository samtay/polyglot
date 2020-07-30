require 'lib/linked_list'

# This question is asked by Facebook. Given a linked list, containing unique
# values, reverse it, and return the result.
#
# @example
#   list = LinkedList.new nil
#   list.from_list([1,2,3])
#   reversed = reverse(list)
#   reversed.value #=> 3
#   iterate_nodes(reversed) #=> [3,2,1]
#   list = LinkedList.new nil
#   list.from_list([7,15,9,2])
#   iterate_nodes(reverse(list)) #=> [2,9,15,7]
#   list = LinkedList.new 1
#   iterate_nodes(reverse(list)) #=> [1]
def reverse(list)
  stack = []
  current = list.head
  while current.next != nil
    stack.push(current.value)
    current = current.next
  end

  last = Node.new(current.value, nil)
  current = last
  while not stack.empty?
    next_node = Node.new(stack.pop(), nil)
    current.next = next_node
    current = next_node
  end

  return last
end

def iterate_nodes(node)
  elements = []
  loop do
    elements << node.value
    node = node.next
    break if node == nil
  end
  return elements
end
