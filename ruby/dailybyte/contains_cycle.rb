require 'lib/linked_list'

# @example
#   list = LinkedList.new 1
#   list.add_value(2)
#   node3 = Node.new(3, list.head)
#   list.add_node(node3)
#   contains_cycle(list) #=> true
#   list.from_list([1,2,3])
#   contains_cycle(list) #=> false
def contains_cycle(list)
  node_pointers = Hash.new
  current_node = list.head
  while current_node.next != nil
    ptr = current_node.object_id << 1
    if node_pointers.member?(ptr)
      return true
    else
      node_pointers[ptr] = true
      current_node = current_node.next
    end
  end
  return false
end
