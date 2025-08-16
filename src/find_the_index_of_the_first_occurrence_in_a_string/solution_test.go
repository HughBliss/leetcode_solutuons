package findtheindexofthefirstoccurrenceinastring_test

import (
	"testing"

	"github.com/go-playground/assert/v2"
)

func strStr(haystack string, needle string) int {

	for i := range len(haystack) {
		for j := range len(needle) {
			if j+i >= len(haystack) {
				return -1 // выходим если слово не вписывается
			}
			if haystack[i+j] != needle[j] {
				break
			}
			if j == len(needle)-1 {
				return i
			}
		}
	}

	return -1
}

func Test(t *testing.T) {
	t.Run("case1", func(t *testing.T) {
		occurance := strStr("leetcode", "leeto")
		assert.Equal(t, -1, occurance)
	})
	t.Run("case2", func(t *testing.T) {
		occurance := strStr("sadbutsad", "sad")
		assert.Equal(t, 0, occurance)
	})
	t.Run("case3", func(t *testing.T) {
		occurance := strStr("sa", "sad")
		assert.Equal(t, -1, occurance)
	})
}
