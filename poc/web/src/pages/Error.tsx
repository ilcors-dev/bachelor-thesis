import { useRouteError } from 'react-router-dom';

export const Error = () => {
	const error = useRouteError() as any;
	console.error(error);

	return (
		<div
			id="error-page"
			className="inline-flex h-screen w-full items-center justify-center"
		>
			<div className="mb-28 text-center">
				<h1 className="text-4xl font-bold">{error.status}</h1>
				<p className="text-2xl font-semibold">{error.statusText}.</p>
			</div>
		</div>
	);
};
