#include <iostream>

#include "SinglyLinkedList.h"

int main() {
  SinglyLinkedList<int> empty{};
  SinglyLinkedList<int> list{};
  for (int i{0}; i < 10; ++i) {
    list.push(i);
  }
  SinglyLinkedList<int> list2{list};
  for (int i{0}; i < 3; ++i) {
    std::cout << "Popped: " << *list2.pop() << '\n';
  }
  auto list3{list};
  list3.pop();
  empty.printList();
  list.printList();
  list2.printList();
  list3.printList();

  return 0;
}