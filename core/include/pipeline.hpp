#pragma once


#include <queue>
// #include <mutex>

namespace rscamera {
template <typename T> class Pipeline {
    private:
	std::queue<T> queue_;
	// std::mutex mutex_;

    public:
	void push( T && value ) {
		// std::unique_lock<std::mutex> lock( mutex_ );
		queue_.push( std::move( value ) );
	}

	bool empty() {
		// std::unique_lock<std::mutex> lock( mutex_ );
		return queue_.size() <= 0;
	}

	size_t count() {
		// std::unique_lock<std::mutex> lock( mutex_ );
		return queue_.size();
	}

	bool pop( T & value ) {
		// std::unique_lock<std::mutex> lock( mutex_ );

		if ( queue_.empty() )
			return false;

		value = std::move( queue_.front() );
		queue_.pop();
		return true;
	}
};
}