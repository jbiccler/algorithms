#include <iostream>
#include <optional>

template <typename T>
class Node {
 private:
  T m_data{};
  Node* next{};

 public:
  Node() = default;
  Node(T& data) : m_data{data} {};

  void setNext(Node<T>* ptr) { next = ptr; }
  void clearNext() { next = nullptr; }
  T getData() const { return m_data; }
  Node* getNext() const { return next; }
};

template <typename T>
class SinglyLinkedList {
 private:
  Node<T>* m_head{nullptr};
  Node<T>* m_tail{nullptr};
  int32_t m_len{0};

 public:
  SinglyLinkedList() = default;
  void push(T data);
  std::optional<T> pop();
  int32_t len() { return m_len; };
  void printList() const;
  ~SinglyLinkedList() {
    Node<T>* n{m_head};
    while (n) {
      auto tmp{n->getNext()};
      delete n;
      n = tmp;
    }
  }
  SinglyLinkedList(const SinglyLinkedList& l) {
    if (l.m_head) {
      auto n{l.m_head};
      while (n) {
        push(n->getData());
        n = n->getNext();
      }
    }
  }
  SinglyLinkedList<T>& operator=(const SinglyLinkedList& l) {
    // clean up current list
    while (pop().has_value()) {
    }
    if (l.m_head) {
      auto n{l.m_head};
      while (n) {
        push(n->getData());
        n = n->getNext();
      }
    }
  }
};

template <typename T>
void SinglyLinkedList<T>::push(T data) {
  Node<T>* node = new Node(data);
  if (!m_head) {
    // first node
    m_head = node;
    m_tail = node;
  } else {
    m_tail->setNext(node);
    m_tail = node;
  }
  ++m_len;
}

template <typename T>
std::optional<T> SinglyLinkedList<T>::pop() {
  if (!m_tail) {
    return std::nullopt;
  } else if (m_tail == m_head) {
    auto res{m_tail->getData()};
    delete m_tail;
    m_tail = nullptr;
    m_head = nullptr;
    m_len = 0;
    return res;
  } else {
    auto res{m_tail->getData()};
    auto n{m_head};
    while (true) {
      if (n->getNext() == m_tail) {
        break;
      } else {
        n = n->getNext();
      }
    }
    delete m_tail;
    m_tail = n;
    m_tail->clearNext();
    --m_len;
    return res;
  }
};

template <typename T>
void SinglyLinkedList<T>::printList() const {
  std::cout << "Printing list of length: " << m_len << '\n';
  if (m_tail) {
    Node<T>* n{m_head};
    while (n) {
      std::cout << n->getData();
      n = n->getNext();
      if (n) {
        std::cout << " -> ";
      }
    }
  }
  std::cout << '\n';
}
