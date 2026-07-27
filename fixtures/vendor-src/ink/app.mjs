// Ink (React for CLIs) corpus app.
//
// Represents the Node TUI family -- the same family Claude Code itself is
// built on. Uses Ink's Box borders plus an inverse-video selection.
import React, { useState } from 'react';
import { render, Box, Text, useInput, useApp } from 'ink';

const ITEMS = [
  ['api-gateway', 'running', '12ms'],
  ['auth-service', 'running', '31ms'],
  ['billing', 'degraded', '412ms'],
  ['search-index', 'running', '88ms'],
  ['mailer', 'stopped', '-'],
];

const pad = (s, n) => String(s).padEnd(n).slice(0, n);

function App() {
  const [cursor, setCursor] = useState(0);
  const { exit } = useApp();
  useInput((input, key) => {
    if (input === 'q') exit();
    if (input === 'j' || key.downArrow) setCursor((c) => (c + 1) % ITEMS.length);
    if (input === 'k' || key.upArrow) setCursor((c) => (c - 1 + ITEMS.length) % ITEMS.length);
  });

  return React.createElement(
    Box,
    { flexDirection: 'column', width: 78 },
    React.createElement(
      Box,
      { borderStyle: 'round', borderColor: 'cyan', width: 78 },
      React.createElement(Text, { bold: true }, ' Deploy Console — cluster prod-eu-1 ')
    ),
    React.createElement(
      Box,
      { flexDirection: 'row' },
      React.createElement(
        Box,
        { borderStyle: 'round', flexDirection: 'column', width: 46 },
        React.createElement(Text, { bold: true }, `${pad('SERVICE', 15)}${pad('STATE', 11)}${pad('P99', 8)}`),
        ...ITEMS.map(([n, s, l], i) =>
          React.createElement(
            Text,
            { key: n, inverse: i === cursor },
            `${pad(n, 15)}${pad(s, 11)}${pad(l, 8)}`
          )
        )
      ),
      React.createElement(
        Box,
        { borderStyle: 'round', flexDirection: 'column', width: 32 },
        React.createElement(Text, null, 'events'),
        React.createElement(Text, null, '• deploy started'),
        React.createElement(Text, null, '• image pulled'),
        React.createElement(Text, null, '• health check ok')
      )
    ),
    React.createElement(Text, { inverse: true }, pad(' j/k move   q quit ', 78))
  );
}

render(React.createElement(App));
