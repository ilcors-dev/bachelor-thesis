import { useAutoAnimate } from '@formkit/auto-animate/react';
import { TrashIcon } from '@heroicons/react/20/solid';
import axios from 'axios';
import moment from 'moment';
import { useQuery } from 'react-query';
import { Link } from 'react-router-dom';
import { useSession } from '../hooks/useSession';
import { ChatCardSkeleton } from './ChatCardSkeleton';
import { ChatLoading } from './ChatLoading';

export const ChatList = () => {
	const session = useSession().offlineGet();
	const [animationParent] = useAutoAnimate();

	const { isLoading, data, error, refetch } = useQuery<Chat[]>(
		['chats'],
		async () => {
			try {
				const response = await axios.get('/api/chats');

				const chats = response.data as Chat[];

				return chats;
			} catch (error) {
				console.error(error);
				return [];
			}
		}
	);

	const deleteChat = async (chatId: number) => {
		try {
			await axios.delete(`/api/chats/${chatId}`);

			refetch();
		} catch (error) {
			console.error(error);
		}
	};

	return (
		<div>
			<ul
				className="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3"
				ref={animationParent}
			>
				{isLoading && <ChatLoading />}
				{!data && !isLoading && <li>No chats available</li>}
				{data &&
					data.map((chat, i) => (
						<Link key={i} to={`chats/${chat.id}`} className="h-full">
							<ChatCardSkeleton className="relative h-full">
								<div>
									{session?.session_id === chat.created_by.session_id && (
										<button
											onClick={(e) => {
												e.preventDefault();
												e.stopPropagation();
												deleteChat(chat.id);
											}}
											className="group absolute top-2 right-2 my-auto flex h-min cursor-pointer items-center justify-center rounded-lg border border-transparent bg-white p-3 text-center text-sm font-medium text-black hover:bg-gray-100 hover:shadow focus:z-10 focus:ring-4 focus:!ring-2 focus:ring-blue-300 disabled:hover:bg-blue-700 dark:bg-blue-600 dark:text-white dark:hover:bg-blue-700 dark:focus:ring-blue-800 dark:disabled:hover:bg-blue-600 sm:p-2 sm:text-base"
										>
											<TrashIcon className="h-4 w-4 text-red-700" />
										</button>
									)}
									<div className="flex h-full max-w-sm flex-col justify-between">
										<div>
											<h1 className="text-xl font-semibold">{chat.name}</h1>
											<p>{chat.description}</p>
										</div>
										<div className="mt-4 flex flex-col text-gray-700">
											<small>
												Created by: {chat.created_by.emoji}{' '}
												{chat.created_by.name}
												{session?.session_id === chat.created_by.session_id &&
													' (you)'}
											</small>
											<small>{moment(chat.created_at).toLocaleString()}</small>
										</div>
									</div>
								</div>
							</ChatCardSkeleton>
						</Link>
					))}
			</ul>
		</div>
	);
};
