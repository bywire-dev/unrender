// Bubbletea + Lipgloss corpus app.
//
// Represents the Go TUI family. Deliberately uses Lipgloss's rounded borders
// and a background-painted selection, which is the idiom the borderless /
// paint-driven half of the unrenderer has to cope with.
package main

import (
	"fmt"
	"os"
	"strings"

	tea "github.com/charmbracelet/bubbletea"
	"github.com/charmbracelet/lipgloss"
)

var (
	panelStyle = lipgloss.NewStyle().
			Border(lipgloss.RoundedBorder()).
			BorderForeground(lipgloss.Color("63")).
			Padding(0, 1)

	selStyle = lipgloss.NewStyle().
			Background(lipgloss.Color("62")).
			Foreground(lipgloss.Color("230"))

	headerStyle = lipgloss.NewStyle().Bold(true)

	statusStyle = lipgloss.NewStyle().
			Background(lipgloss.Color("240")).
			Foreground(lipgloss.Color("255")).
			Width(78)
)

type item struct {
	name   string
	state  string
	region string
}

var items = []item{
	{"api-gateway", "running", "eu-1"},
	{"auth-service", "running", "eu-1"},
	{"billing", "degraded", "us-1"},
	{"search-index", "running", "us-1"},
	{"mailer", "stopped", "eu-1"},
}

type model struct{ cursor int }

func (m model) Init() tea.Cmd { return nil }

func (m model) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	if k, ok := msg.(tea.KeyMsg); ok {
		switch k.String() {
		case "q", "ctrl+c":
			return m, tea.Quit
		case "j", "down":
			m.cursor = (m.cursor + 1) % len(items)
		case "k", "up":
			m.cursor = (m.cursor - 1 + len(items)) % len(items)
		}
	}
	return m, nil
}

func (m model) View() string {
	var rows []string
	rows = append(rows, headerStyle.Render(fmt.Sprintf("%-14s %-10s %-6s", "SERVICE", "STATE", "REGION")))
	for i, it := range items {
		line := fmt.Sprintf("%-14s %-10s %-6s", it.name, it.state, it.region)
		if i == m.cursor {
			line = selStyle.Render(line)
		}
		rows = append(rows, line)
	}
	left := panelStyle.Width(36).Render(strings.Join(rows, "\n"))

	detail := []string{
		"name:    " + items[m.cursor].name,
		"state:   " + items[m.cursor].state,
		"region:  " + items[m.cursor].region,
		"replicas: 3",
	}
	right := panelStyle.Width(30).Render(strings.Join(detail, "\n"))

	body := lipgloss.JoinHorizontal(lipgloss.Top, left, right)
	status := statusStyle.Render(" j/k move   q quit ")
	return body + "\n" + status + "\n"
}

func main() {
	p := tea.NewProgram(model{})
	if _, err := p.Run(); err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
}
