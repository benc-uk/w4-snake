package main

import "cart/w4" //nolint:all

var (
	snake = &Snake{
		Body: []Point{
			{X: 2, Y: 0},
			{X: 1, Y: 0},
			{X: 0, Y: 0},
		},
		Direction: Point{X: 1, Y: 0},
	}
	frameCount = 0
)

type Point struct {
	X int
	Y int
}

type Snake struct {
	Body      []Point
	Direction Point
}

func (s *Snake) Draw() {
	*w4.DRAW_COLORS = 0x0042
	for _, part := range s.Body {
		w4.Rect(part.X*8, part.Y*8, 8, 8)
	}

	*w4.DRAW_COLORS = 0x0004
	w4.Rect(s.Body[0].X*8, s.Body[0].Y*8, 8, 8)
}

func (s *Snake) Update() {
	for i := len(s.Body) - 1; i > 0; i-- {
		s.Body[i] = s.Body[i-1]
	}
	s.Body[0].X = (s.Body[0].X + s.Direction.X) % 20
	s.Body[0].Y = (s.Body[0].Y + s.Direction.Y) % 20
	if s.Body[0].X < 0 {
		s.Body[0].X = 19
	}
	if s.Body[0].Y < 0 {
		s.Body[0].Y = 19
	}
}

//go:export start
func start() {
	w4.PALETTE[0] = 0x2d1b00
	w4.PALETTE[1] = 0x1e606e
	w4.PALETTE[2] = 0x5ab9a8
	w4.PALETTE[3] = 0xc4f0c2
}

//go:export update
func update() {
	frameCount++
	if frameCount%9 == 0 {
		snake.Update()
	}
	snake.Draw()
}
