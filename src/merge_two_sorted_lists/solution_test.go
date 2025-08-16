/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */

package mergetwosortedlists

type ListNode struct {
	Val  int
	Next *ListNode
}

func mergeTwoLists(list1 *ListNode, list2 *ListNode) *ListNode {

	if list1 == nil && list2 == nil {
		return nil
	}

	if list1 == nil && list2 != nil {
		return list2
	}

	if list1 != nil && list2 == nil {
		return list1
	}

	if list1.Val >= list2.Val {
		return &ListNode{
			Val:  list2.Val,
			Next: mergeTwoLists(list1, list2.Next),
		}
	}

	return &ListNode{
		Val:  list1.Val,
		Next: mergeTwoLists(list1.Next, list2),
	}

}
