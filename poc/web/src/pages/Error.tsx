import { useRouteError } from 'react-router-dom';

export const Error = () => {
	const error = useRouteError();
	console.error(error);

	return (
		<div id="error-page" className="h-screen w-screen ">
			<h1>Oops! Something went wrong.</h1>
			<p>
				<a href="/">Go back to the home page</a>
			</p>
		</div>
	);
};
