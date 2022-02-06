CREATE TABLE blog_posts (
	id SERIAL PRIMARY KEY,
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMP WITH TIME ZONE,
	published_at TIMESTAMP WITH TIME ZONE,
	publish_state VARCHAR(255) NOT NULL DEFAULT 'draft',
	body TEXT,
	title VARCHAR(255) NOT NULL,
	slug VARCHAR(255) UNIQUE NOT NULL
);