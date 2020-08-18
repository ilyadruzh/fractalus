struct node
{
	el_type element;         //”казатель, содержащий элемент стека
	struct node *next;       //”казатель на следующий узел
};
     
typedef struct node node;    //ƒл€ удобства введЄм короткое обозначение дл€ struct node

node *push(node *stack, el_type element);
el_type pop(node **pstack);
