#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdbool.h>

typedef struct Element {
  struct Element *next;
  void *data;
} Element;

bool createStack(Element **stack) {
  *stack = NULL;
  return true;
}

bool deleteStack(Element **stack) {

}
bool push(Element **stack, void *data) {
  Element *elem = malloc(sizeof(Element));
  if (!elem) return false;
  elem->data = data;
  elem->next = *stack;
  *stack = elem;
  return true;
}

bool pop(Element **stack, void **data) {
  Element *elem;
  if (!(elem = *stack)) return false;
  *data = elem->data;
  *stack = elem->next;
  free(elem);
  return true;
}

int main( ) {
  // some data
  int a = 10;
  int c = 11;
  int d = 7249424;

  // stack elements
  Element node;
  Element* nodeptr = &node;
  createStack(&nodeptr);
  push(&nodeptr, &a);
  push(&nodeptr, &c);
  push(&nodeptr, &d);

  void *curr;
  while (pop(&nodeptr, &curr)) {
    printf("%d\n", *(int *) curr);
  }

   return 0;
}
