import { PlusIcon } from '@heroicons/react/20/solid';
import { Link } from 'react-router-dom';
import { ChatList } from '../../components/ChatList';

export const Index = (): JSX.Element => {
	return (
		<>
			<div className="relative h-screen pt-8">
				<div>
					<h1 className="text-4xl font-bold">Welcome to Wasi PoC</h1>
					<p>The following client is interacting with a wasi backend!</p>
				</div>

				<section className="mt-8">
					<ChatList />
				</section>
				<section className="mt-6">
					<Link
						to="/chats/create"
						className="hidden items-center justify-center rounded-lg bg-blue-700 p-2 text-sm font-medium text-white hover:bg-blue-800 focus:outline-none focus:ring-4 focus:ring-blue-300 dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800 sm:inline-flex sm:w-full md:w-auto"
					>
						<span>Create chat</span>
						<PlusIcon className="h-6 w-6" />
					</Link>
				</section>
				<div className="absolute bottom-1/3 right-2 block sm:hidden">
					<Link
						to="/chats/create"
						className="flex items-center rounded-lg bg-blue-700 p-4 text-sm font-medium text-white hover:bg-blue-800 focus:outline-none focus:ring-4 focus:ring-blue-300 dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
					>
						<PlusIcon className="h-6 w-6" />
					</Link>
				</div>
			</div>
		</>
	);
};
