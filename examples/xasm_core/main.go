package xasm_core

import (
	"fmt"
	"math"

	. "github.com/adam-mcdaniel/xgopher"
)

func Xasm_list(m *Machine) {
	result := NewEmptyList().Slice()

	value := m.Pop()
	count := value.Number()

	for ; count > 0; count-- {
		result = append(result, m.Pop())
	}

	m.Push(NewList(result))
}

func Xasm_range(m *Machine) {
	result := NewEmptyList().Slice()

	lower := m.Pop().Number()
	upper := m.Pop().Number()

	for count := lower; count < upper; count++ {
		result = append(result, NewNumber(count))
	}

	m.Push(NewList(result))
}

func Xasm_filter(m *Machine) {
	result := NewEmptyList().Slice()

	list := m.Pop().Slice()
	function := m.Pop()

	for _, item := range list {
		m.Push(item)
		m.Push(function)
		m.Call()
		if m.Pop().Bool() {
			result = append(result, item)
		}
	}

	m.Push(NewList(result))
}

func Xasm_reduce(m *Machine) {
	list := m.Pop().Slice()
	function := m.Pop()
	accumulator := m.Pop()

	for _, item := range list {
		m.Push(accumulator)
		m.Push(item)
		m.Push(function)
		m.Call()
		accumulator = m.Pop()
	}

	m.Push(accumulator)
}

func Xasm_map(m *Machine) {
	result := NewEmptyList().Slice()

	list := m.Pop().Slice()
	function := m.Pop()

	for _, item := range list {
		m.Push(item)
		m.Push(function)
		m.Call()
		result = append(result, m.Pop())
	}

	m.Push(NewList(result))
}

func Xasm_reverse(m *Machine) {
	a := m.Pop().Slice()

	for i := len(a)/2 - 1; i >= 0; i-- {
		opp := len(a) - 1 - i
		a[i], a[opp] = a[opp], a[i]
	}

	m.Push(NewList(a))
}

func Xasm_push(m *Machine) {
	list := m.Pop()
	item := m.Pop()
	m.Push(NewList(append(list.Slice(), item)))
}

func Xasm_pop(m *Machine) {
	value := m.Pop()
	list := value.Slice()
	if len(list) > 0 {
		item := list[len(list)-1]
		list = list[:len(list)-1]
		m.Push(item)
	} else {
		m.Push(NewNone())
	}
	m.Push(NewList(list))
}

func Xasm_len(m *Machine) {
	value := m.Pop()
	l := value.Slice()
	s := value.Str()
	n := math.Max(float64(len(l)), float64(len(s)))
	m.Push(NewNumber(n))
}

func Xasm_format(m *Machine) {
	m.Push(NewString(fmt.Sprintf("%v", m.Pop())))
}

func Xasm_debug(m *Machine) {
	m.Push(NewString(fmt.Sprintf("%v", *m)))
}

func Xasm_new(m *Machine) {
	m.Call()
	m.Push(NewString("new"))
	m.MethodCall()
}

func Xasm_add(m *Machine) {
	a := m.Pop()
	b := m.Pop()
	m.Push(a.Add(b))
}

func Xasm_sub(m *Machine) {
	a := m.Pop()
	b := m.Pop()
	m.Push(a.Sub(b))
}

func Xasm_mul(m *Machine) {
	a := m.Pop()
	b := m.Pop()
	m.Push(a.Mul(b))
}

func Xasm_div(m *Machine) {
	a := m.Pop()
	b := m.Pop()
	m.Push(a.Div(b))
}

func Xasm_rem(m *Machine) {
	a := m.Pop()
	b := m.Pop()
	m.Push(a.Rem(b))
}

func Xasm_not(m *Machine) {
	m.Push(m.Pop().Not())
}

func Xasm_eq(m *Machine) {
	m.Push(m.Pop().Eq(m.Pop()))
}
