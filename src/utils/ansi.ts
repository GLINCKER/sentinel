/**
 * ANSI color code parser for terminal output
 *
 * Converts ANSI escape sequences to HTML spans with appropriate classes.
 * Supports basic 16 colors and common text formatting.
 *
 * Part of Sentinel - Your Development Guardian
 * Built by Glincker (A GLINR Product)
 *
 * @see https://glincker.com/sentinel
 */

export interface AnsiSegment {
  text: string;
  classes: string[];
}

const ANSI_COLORS: Record<string, string> = {
  '30': 'ansi-black',
  '31': 'ansi-red',
  '32': 'ansi-green',
  '33': 'ansi-yellow',
  '34': 'ansi-blue',
  '35': 'ansi-magenta',
  '36': 'ansi-cyan',
  '37': 'ansi-white',
  '90': 'ansi-bright-black',
  '91': 'ansi-bright-red',
  '92': 'ansi-bright-green',
  '93': 'ansi-bright-yellow',
  '94': 'ansi-bright-blue',
  '95': 'ansi-bright-magenta',
  '96': 'ansi-bright-cyan',
  '97': 'ansi-bright-white',
  '40': 'ansi-bg-black',
  '41': 'ansi-bg-red',
  '42': 'ansi-bg-green',
  '43': 'ansi-bg-yellow',
  '44': 'ansi-bg-blue',
  '45': 'ansi-bg-magenta',
  '46': 'ansi-bg-cyan',
  '47': 'ansi-bg-white'
};

const ANSI_STYLES: Record<string, string> = {
  '1': 'ansi-bold',
  '2': 'ansi-dim',
  '3': 'ansi-italic',
  '4': 'ansi-underline'
};

/**
 * Parse ANSI escape sequences and return HTML-safe string
 */
export function parseAnsi(text: string): string {
  const segments = parseAnsiToSegments(text);

  return segments
    .map(segment => {
      if (segment.classes.length === 0) {
        return escapeHtml(segment.text);
      }
      return `<span class="${segment.classes.join(' ')}">${escapeHtml(segment.text)}</span>`;
    })
    .join('');
}

/**
 * Parse ANSI text into segments with classes
 */
export function parseAnsiToSegments(text: string): AnsiSegment[] {
  const segments: AnsiSegment[] = [];
  // eslint-disable-next-line no-control-regex
  const ansiRegex = /\x1b\[([0-9;]*)m/g;

  let currentClasses: string[] = [];
  let lastIndex = 0;
  let match: RegExpExecArray | null;

  while ((match = ansiRegex.exec(text)) !== null) {
    // Add text before this escape sequence
    if (match.index > lastIndex) {
      const text = text.substring(lastIndex, match.index);
      if (text.length > 0) {
        segments.push({ text, classes: [...currentClasses] });
      }
    }

    // Parse escape codes
    const codes = match[1].split(';').filter(c => c.length > 0);

    for (const code of codes) {
      if (code === '0' || code === '') {
        // Reset
        currentClasses = [];
      } else if (ANSI_COLORS[code]) {
        currentClasses.push(ANSI_COLORS[code]);
      } else if (ANSI_STYLES[code]) {
        currentClasses.push(ANSI_STYLES[code]);
      }
    }

    lastIndex = match.index + match[0].length;
  }

  // Add remaining text
  if (lastIndex < text.length) {
    segments.push({
      text: text.substring(lastIndex),
      classes: [...currentClasses]
    });
  }

  return segments;
}

/**
 * Escape HTML special characters
 */
function escapeHtml(text: string): string {
  const div = document.createElement('div');
  div.textContent = text;
  return div.innerHTML;
}

/**
 * Strip all ANSI codes from text
 */
export function stripAnsi(text: string): string {
  // eslint-disable-next-line no-control-regex
  return text.replace(/\x1b\[[0-9;]*m/g, '');
}

/**
 * Detect log level from line content
 */
export function detectLogLevel(line: string): 'error' | 'warn' | 'info' | 'debug' | null {
  const lower = line.toLowerCase();

  if (lower.includes('error') || lower.includes('err') || lower.includes('fatal')) {
    return 'error';
  }
  if (lower.includes('warn') || lower.includes('warning')) {
    return 'warn';
  }
  if (lower.includes('debug') || lower.includes('trace')) {
    return 'debug';
  }
  if (lower.includes('info')) {
    return 'info';
  }

  return null;
}
