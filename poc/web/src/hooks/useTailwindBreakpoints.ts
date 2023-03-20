import { useEffect, useState } from 'react';

const screens: { [key: string]: number } = {
	sm: 640,
	md: 768,
	lg: 1024,
	xl: 1280,
	'2xl': 1536,
};

export const useTailwindBreakpoints = () => {
	const [breakpoint, setBreakpoint] = useState<string>('sm');

	const handleResize = () => {
		const width = window.innerWidth;
		const newBreakpoint =
			Object.keys(screens).find((key) => width < screens[key]) || '2xl';
		setBreakpoint(newBreakpoint);
	};

	useEffect(() => {
		handleResize();
		window.addEventListener('resize', handleResize);
		return () => window.removeEventListener('resize', handleResize);
	}, []);

	return breakpoint;
};

export const is = (breakpoint: 'sm' | 'md' | 'lg' | 'xl' | '2xl') => {
	const currentBreakpoint = useTailwindBreakpoints();

	return currentBreakpoint === breakpoint;
};

export const greater = (breakpoint: 'sm' | 'md' | 'lg' | 'xl' | '2xl') => {
	const currentBreakpoint = useTailwindBreakpoints();

	return screens[currentBreakpoint] > screens[breakpoint];
};
