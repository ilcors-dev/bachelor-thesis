import axios from 'axios';
import moment from 'moment';
import { useQuery } from 'react-query';
import { Link } from 'react-router-dom';
import { useSession } from '../hooks/useSession';
import { ChatCardSkeleton } from './ChatCardSkeleton';
import { ChatLoading } from './ChatLoading';

export const ChatList = () => {
	const session = useSession().offlineGet();

	const { isLoading, data, error } = useQuery<Chat[]>(['chats'], async () => {
		try {
			const response = await axios.get('/api/chats');

			const chats = response.data as Chat[];

			return chats;
		} catch (error) {
			console.error(error);
			return [];
		}
	});

	return (
		<div>
			<ul className="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3">
				{isLoading && <ChatLoading />}
				{!data && !isLoading && <li>No chats available</li>}
				{data &&
					data.map((chat, i) => (
						<Link key={i} to={`chats/${chat.id}`} className="h-full">
							<ChatCardSkeleton className="h-full">
								<div className="flex h-full max-w-sm flex-col justify-between">
									<div>
										<h1 className="text-xl font-semibold">{chat.name}</h1>
										<p>{chat.description}</p>
									</div>
									<div className="mt-4 flex flex-col text-gray-700">
										<small>
											Created by: {chat.created_by.emoji} {chat.created_by.name}
											{session?.session_id === chat.created_by.session_id &&
												' (you)'}
										</small>
										<small>{moment(chat.created_at).toLocaleString()}</small>
									</div>
								</div>
							</ChatCardSkeleton>
						</Link>
					))}
			</ul>
		</div>
	);
};
