class Node:

    def __init__(self, key: int = -1, val: int = -1, prev: 'Node' = None, next: 'Node' = None) -> None:
        self.key = key
        self.val = val
        self.prev = prev
        self.next = next

class LRUCache:

    def __init__(self, capacity: int):
        self.map = dict()
        self.capacity = capacity

        # boundary nodes
        self.head = Node()
        self.tail = Node()
        self.head.next = self.tail
        self.tail.prev = self.head
    
    def remove(self, node: Node) -> None:
        prev_node, next_node = node.prev, node.next
        prev_node.next = next_node
        next_node.prev = prev_node
    
    def add_to_tail(self, node: Node) -> None:
        last_node = self.tail.prev
        last_node.next = node
        node.prev = last_node
        node.next = self.tail
        self.tail.prev = node

    def get(self, key: int) -> int:
        if key in self.map:
            curr_node = self.map[key]

            # remove the node
            self.remove(curr_node)

            # add node to the last
            self.add_to_tail(curr_node)

            return curr_node.val
        
        return -1

    def put(self, key: int, value: int) -> None:
        # if key already exists -> remove the node
        if key in self.map:
            self.remove(self.map[key])

        # create a new node and add it to the last
        new_node = Node(key, value)
        self.map[key] = new_node
        self.add_to_tail(new_node)

        # check for capacity
        if len(self.map) > self.capacity:
            lru_node = self.head.next

            # remove the node
            self.remove(lru_node)

            # delete entry from the map
            del self.map[lru_node.key]
