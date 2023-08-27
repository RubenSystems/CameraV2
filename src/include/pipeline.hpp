#pragma once

#include <condition_variable>
#include <queue>
#include <mutex>

namespace rscamera {
template <typename T> class Pipeline {
    private:
	std::queue<T> queue_;
	std::mutex mutex_;

    public:
	void push( T && value ) {
		std::unique_lock<std::mutex> lock( mutex_ );
		queue_.push( std::move( value ) );
	}

	bool pop( T & value ) {
		std::unique_lock<std::mutex> lock( mutex_ );

		if ( queue_.empty() )
			return false;

		value = std::move( queue_.front() );
		queue_.pop();
		return true;
	}
};
}