// Inline SVG icon library — macOS 26 SF Symbols–inspired line icons.
// Each icon is a pure function returning an SVG element at 1em size.

import type { JSX } from "solid-js";

const defaults: JSX.SvgSVGAttributes<SVGSVGElement> = {
  width: "1em",
  height: "1em",
  viewBox: "0 0 24 24",
  fill: "none",
  stroke: "currentColor",
  "stroke-width": "1.8",
  "stroke-linecap": "round" as const,
  "stroke-linejoin": "round" as const,
};

const icon = (paths: string, extra?: Partial<JSX.SvgSVGAttributes<SVGSVGElement>>) => {
  return (props?: JSX.SvgSVGAttributes<SVGSVGElement>) => (
    <svg {...defaults} {...extra} {...props}>
      {/* Using innerHTML since paths is trusted static content */}
      <g innerHTML={paths} />
    </svg>
  );
};

// --- App logo (colorful mini landscape matching the app icon) ---
export const AppLogo = (props?: JSX.SvgSVGAttributes<SVGSVGElement>) => (
  <svg width="1em" height="1em" viewBox="0 0 24 24" {...props}>
    <defs>
      <clipPath id="lr"><rect width="24" height="24" rx="5.4"/></clipPath>
      <linearGradient id="ls" x1="0" y1="0" x2="0" y2="1">
        <stop offset="0%" stop-color="#56CCF2"/>
        <stop offset="70%" stop-color="#BAE6FD"/>
        <stop offset="100%" stop-color="#FDE68A"/>
      </linearGradient>
      <linearGradient id="lh" x1="0" y1="0" x2="0" y2="1">
        <stop offset="0%" stop-color="#34D399"/>
        <stop offset="100%" stop-color="#10B981"/>
      </linearGradient>
    </defs>
    <g clip-path="url(#lr)">
      <rect width="24" height="24" fill="url(#ls)"/>
      <circle cx="17" cy="6.5" r="2.5" fill="#FFF" opacity="0.9"/>
      <path d="M0 15 L4 11 L8 13.5 L12.5 9 L17 12.5 L20 10.5 L24 13 L24 24 L0 24Z" fill="#6EE7B7" opacity="0.6"/>
      <path d="M0 17 L4 13 L9 16 L13 12 L18 15 L22 13 L24 14.5 L24 24 L0 24Z" fill="url(#lh)"/>
    </g>
  </svg>
);

// --- App (stroke version) ---
export const IconPhoto = icon(
  `<rect x="3" y="4" width="18" height="16" rx="2"/>
   <circle cx="9" cy="10" r="2"/>
   <path d="M21 15l-5-5L5 20"/>`
);

// --- Tabs ---
export const IconGrid = icon(
  `<rect x="3" y="3" width="7" height="7" rx="1.5"/>
   <rect x="14" y="3" width="7" height="7" rx="1.5"/>
   <rect x="3" y="14" width="7" height="7" rx="1.5"/>
   <rect x="14" y="14" width="7" height="7" rx="1.5"/>`
);

export const IconSettings = icon(
  `<path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"/>
   <circle cx="12" cy="12" r="3"/>`
);

// --- Actions ---
export const IconRefresh = icon(
  `<path d="M21 2v6h-6"/>
   <path d="M3 12a9 9 0 0 1 15-6.7L21 8"/>
   <path d="M3 22v-6h6"/>
   <path d="M21 12a9 9 0 0 1-15 6.7L3 16"/>`
);

export const IconDownload = icon(
  `<path d="M12 3v12"/>
   <path d="M8 11l4 4 4-4"/>
   <path d="M20 21H4"/>`
);

export const IconTrash = icon(
  `<path d="M3 6h18"/>
   <path d="M8 6V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/>
   <path d="M19 6l-1 14a2 2 0 0 1-2 2H8a2 2 0 0 1-2-2L5 6"/>
   <line x1="10" y1="11" x2="10" y2="17"/>
   <line x1="14" y1="11" x2="14" y2="17"/>`
);

export const IconSetWallpaper = icon(
  `<rect x="2" y="3" width="20" height="14" rx="2"/>
   <path d="M8 21h8"/>
   <path d="M12 17v4"/>
   <path d="M2 13l5-5 4 4 3-3 5 5"/>`
);

// --- Settings sections ---
export const IconGlobe = icon(
  `<circle cx="12" cy="12" r="10"/>
   <path d="M2 12h20"/>
   <path d="M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z"/>`
);

export const IconFolder = icon(
  `<path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/>`
);

export const IconGear = icon(
  `<path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"/>
   <circle cx="12" cy="12" r="3"/>`
);

// --- Toast ---
export const IconCheckCircle = icon(
  `<path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/>
   <polyline points="22 4 12 14.01 9 11.01"/>`
);

export const IconXCircle = icon(
  `<circle cx="12" cy="12" r="10"/>
   <line x1="15" y1="9" x2="9" y2="15"/>
   <line x1="9" y1="9" x2="15" y2="15"/>`
);

// --- Empty state ---
export const IconImageOff = icon(
  `<rect x="3" y="4" width="18" height="16" rx="2" opacity="0.4"/>
   <circle cx="9" cy="10" r="2" opacity="0.4"/>
   <path d="M21 15l-5-5L5 20" opacity="0.4"/>
   <line x1="2" y1="2" x2="22" y2="22"/>`,
  { "stroke-width": "1.6" }
);
