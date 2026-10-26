// Vector icons, ported from the Python runner's ui/icons.py (drawn on an 18 x 18 grid).

function sunRays() {
  let rays = "";
  for (let angle = 0; angle < 360; angle += 45) {
    const rad = (angle * Math.PI) / 180;
    const x1 = 9 + 6 * Math.cos(rad);
    const y1 = 9 + 6 * Math.sin(rad);
    const x2 = 9 + 8 * Math.cos(rad);
    const y2 = 9 + 8 * Math.sin(rad);
    rays += `<line x1="${x1.toFixed(2)}" y1="${y1.toFixed(2)}" x2="${x2.toFixed(2)}" y2="${y2.toFixed(2)}"/>`;
  }
  return rays;
}

const SHAPES = {
  copy: `<rect x="3.5" y="5.5" width="9" height="9.5" rx="2"/><rect x="6.5" y="2.5" width="9" height="9.5" rx="2"/>`,
  popout: `<line x1="5" y1="13" x2="13" y2="5"/><polyline points="7,5 13,5 13,11"/>`,
  dock: `<line x1="13" y1="5" x2="5" y2="13"/><polyline points="5,7 5,13 11,13"/>`,
  sun: `<circle cx="9" cy="9" r="3.5"/>${sunRays()}`,
  moon: `<defs><mask id="moon-cut"><rect width="18" height="18" fill="white"/><circle cx="12.5" cy="6.5" r="5.5" fill="black"/></mask></defs><circle cx="9" cy="9" r="6.5" fill="currentColor" stroke="none" mask="url(#moon-cut)"/>`,
};

/** An SVG element for the named icon, coloured with the current text color. */
export function icon(kind) {
  return `<svg class="icon" viewBox="0 0 18 18" width="16" height="16" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">${SHAPES[kind]}</svg>`;
}
