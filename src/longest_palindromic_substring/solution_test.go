package solution_test

import (
	"testing"

	"github.com/go-playground/assert/v2"
)

func longestPalindrome(s string) string {

	var longest string

	for start := 0; start < len(s); start++ {
		for end := start; end < len(s); end++ {
			var is_poly = false
			for i := start; i <= (end+start)/2; i++ {
				if s[i] != s[end-(i-start)] {
					is_poly = false
					break
				}
				is_poly = true
			}
			if is_poly && end-start+1 > len(longest) {
				longest = s[start : end+1]
			}
		}
	}
	return longest
}

func Test(t *testing.T) {
	t.Run("case1", func(t *testing.T) {
		res := longestPalindrome("babad")
		assert.Equal(t, res, "bab")
	})
	t.Run("case2", func(t *testing.T) {
		res := longestPalindrome("cbbd")
		assert.Equal(t, res, "bb")
	})
	t.Run("case2", func(t *testing.T) {
		res := longestPalindrome("aacabdkacaa")
		assert.Equal(t, res, "aca")
	})
}
