require 'lib/linked_list'

# @example
#   list = LinkedList.new nil
#   list.from_list([1,2,3,4])
#   node2 = list.nth_node(1)
#   node5 = Node.new(5, node2)
#   list.add_node(node5)
#   get_start_cycle(list).value #=> 2
#   list = LinkedList.new nil
#   list.from_list([1,2,3])
#   get_start_cycle(list) #=> nil
def get_start_cycle(list)
  node_pointers = Hash.new
  current_node = list.head
  while current_node.next != nil
    ptr = current_node.object_id << 1
    if node_pointers.member?(ptr)
      return current_node
    else
      node_pointers[ptr] = true
      current_node = current_node.next
    end
  end
  return nil
end
